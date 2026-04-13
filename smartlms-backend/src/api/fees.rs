// Fees API routes - Fee management, M-Pesa payments, callbacks
use crate::services::fee::{
    self, FeeStructure, MpesaCallbackBody, MpesaConfig, MpesaEnvironment, MpesaPaymentRequest,
    MpesaService, Payment, PaymentMethod, StudentFee,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// Application state for fee services
pub struct FeeAppState {
    pub pool: PgPool,
    pub mpesa_service: MpesaService,
}

impl Clone for FeeAppState {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            mpesa_service: MpesaService::new(MpesaConfig {
                consumer_key: self.mpesa_service.config.consumer_key.clone(),
                consumer_secret: self.mpesa_service.config.consumer_secret.clone(),
                short_code: self.mpesa_service.config.short_code.clone(),
                passkey: self.mpesa_service.config.passkey.clone(),
                environment: self.mpesa_service.config.environment,
                callback_url: self.mpesa_service.config.callback_url.clone(),
            }),
        }
    }
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct CreateFeeStructureRequest {
    pub name: String,
    pub description: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub fee_type: String,
    pub academic_year: String,
    pub semester: Option<String>,
    pub due_date: chrono::DateTime<chrono::Utc>,
    pub is_optional: bool,
    pub late_fee_amount: Option<i64>,
    pub late_fee_grace_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AssignFeeRequest {
    pub student_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct InitiatePaymentRequest {
    pub student_fee_id: Uuid,
    pub phone_number: String,
    pub amount: i64,
    pub account_reference: String,
    pub transaction_desc: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub status: String,
    pub transaction_id: String,
    pub checkout_request_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct FeeStructureResponse {
    pub id: Uuid,
    pub name: String,
    pub amount: i64,
    pub currency: String,
    pub fee_type: String,
    pub due_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct StudentFeeResponse {
    pub id: Uuid,
    pub amount: i64,
    pub amount_paid: i64,
    pub balance: i64,
    pub status: String,
    pub due_date: chrono::DateTime<chrono::Utc>,
}

// Route handlers

/// Create fee structure for institution
pub async fn create_fee_structure(
    State(state): State<FeeAppState>,
    Json(req): Json<CreateFeeStructureRequest>,
) -> Result<Json<FeeStructureResponse>, (StatusCode, String)> {
    // TODO: Extract institution_id from auth context
    let institution_id = Uuid::new_v4(); // Placeholder

    let fee_type = match req.fee_type.as_str() {
        "tuition" => fee::FeeType::Tuition,
        "registration" => fee::FeeType::Registration,
        "library" => fee::FeeType::Library,
        "lab" => fee::FeeType::Lab,
        "accommodation" => fee::FeeType::Accommodation,
        "examination" => fee::FeeType::Examination,
        "certificate" => fee::FeeType::Certificate,
        _ => fee::FeeType::Other,
    };

    let create_req = fee::CreateFeeStructureRequest {
        name: req.name,
        description: req.description,
        amount: req.amount,
        currency: req.currency,
        fee_type,
        academic_year: req.academic_year,
        semester: req.semester,
        due_date: req.due_date,
        is_optional: req.is_optional,
        late_fee_amount: req.late_fee_amount,
        late_fee_grace_days: req.late_fee_grace_days,
    };

    match fee::service::create_fee_structure(&state.pool, institution_id, &create_req).await {
        Ok(fee_struct) => Ok(Json(FeeStructureResponse {
            id: fee_struct.id,
            name: fee_struct.name,
            amount: fee_struct.amount,
            currency: fee_struct.currency,
            fee_type: format!("{:?}", fee_struct.fee_type),
            due_date: fee_struct.due_date,
        })),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// Assign fee to student
pub async fn assign_fee_to_student(
    State(state): State<FeeAppState>,
    Path(fee_structure_id): Path<Uuid>,
    Json(req): Json<AssignFeeRequest>,
) -> Result<Json<StudentFeeResponse>, (StatusCode, String)> {
    match fee::service::assign_fee_to_student(&state.pool, fee_structure_id, req.student_id).await {
        Ok(student_fee) => Ok(Json(StudentFeeResponse {
            id: student_fee.id,
            amount: student_fee.amount,
            amount_paid: student_fee.amount_paid,
            balance: student_fee.balance,
            status: format!("{:?}", student_fee.status),
            due_date: student_fee.due_date,
        })),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// Get student fees
pub async fn get_student_fees(
    State(state): State<FeeAppState>,
    Path(student_id): Path<Uuid>,
) -> Result<Json<Vec<StudentFeeResponse>>, (StatusCode, String)> {
    match fee::service::get_student_fees(&state.pool, student_id).await {
        Ok(fees) => {
            let response: Vec<StudentFeeResponse> = fees
                .iter()
                .map(|f| StudentFeeResponse {
                    id: f.id,
                    amount: f.amount,
                    amount_paid: f.amount_paid,
                    balance: f.balance,
                    status: format!("{:?}", f.status),
                    due_date: f.due_date,
                })
                .collect();
            Ok(Json(response))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// Initiate M-Pesa payment
pub async fn initiate_mpesa_payment(
    State(state): State<FeeAppState>,
    Json(req): Json<InitiatePaymentRequest>,
) -> Result<Json<PaymentResponse>, (StatusCode, String)> {
    let mpesa_req = MpesaPaymentRequest {
        phone_number: req.phone_number,
        amount: req.amount,
        account_reference: req.account_reference,
        transaction_desc: req.transaction_desc,
    };

    match fee::service::process_mpesa_payment(
        &state.pool,
        &state.mpesa_service,
        req.student_fee_id,
        &mpesa_req,
    )
    .await
    {
        Ok(payment) => {
            let checkout_id = payment.gateway_response.as_ref().and_then(|resp| {
                serde_json::from_str::<serde_json::Value>(resp)
                    .ok()
                    .and_then(|v| v["checkout_request_id"].as_str().map(String::from))
            });

            Ok(Json(PaymentResponse {
                id: payment.id,
                status: format!("{:?}", payment.status),
                transaction_id: payment.transaction_id,
                checkout_request_id: checkout_id,
                message: "STK Push sent successfully. Check your phone.".to_string(),
            }))
        }
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// M-Pesa callback handler (webhook from Safaricom)
pub async fn mpesa_callback(
    State(state): State<FeeAppState>,
    Json(callback_body): Json<MpesaCallbackBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    tracing::info!("Received M-Pesa callback: {:?}", callback_body);

    match fee::service::process_mpesa_callback(&state.pool, &callback_body.stk_callback).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "ResultCode": 0,
            "ResultDesc": "Accepted"
        }))),
        Err(e) => {
            tracing::error!("Failed to process M-Pesa callback: {}", e);
            Ok(Json(serde_json::json!({
                "ResultCode": 1,
                "ResultDesc": e
            })))
        }
    }
}

/// Query payment status
pub async fn get_payment_status(
    State(state): State<FeeAppState>,
    Path(payment_id): Path<Uuid>,
) -> Result<Json<PaymentResponse>, (StatusCode, String)> {
    match fee::service::get_payment(&state.pool, payment_id).await {
        Ok(Some(payment)) => Ok(Json(PaymentResponse {
            id: payment.id,
            status: format!("{:?}", payment.status),
            transaction_id: payment.transaction_id,
            checkout_request_id: None,
            message: "".to_string(),
        })),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Payment not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// Get fee structures for institution
pub async fn get_institution_fee_structures(
    State(state): State<FeeAppState>,
) -> Result<Json<Vec<FeeStructureResponse>>, (StatusCode, String)> {
    // TODO: Extract institution_id from auth context
    let institution_id = Uuid::new_v4(); // Placeholder

    match fee::service::get_institution_fee_structures(&state.pool, institution_id).await {
        Ok(fees) => {
            let response: Vec<FeeStructureResponse> = fees
                .iter()
                .map(|f| FeeStructureResponse {
                    id: f.id,
                    name: f.name.clone(),
                    amount: f.amount,
                    currency: f.currency.clone(),
                    fee_type: format!("{:?}", f.fee_type),
                    due_date: f.due_date,
                })
                .collect();
            Ok(Json(response))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// Create router
pub fn fees_router() -> Router {
    Router::new()
        .route("/structures", post(create_fee_structure))
        .route("/structures", get(get_institution_fee_structures))
        .route("/structures/:id/assign", post(assign_fee_to_student))
        .route("/student/:student_id", get(get_student_fees))
        .route("/pay/mpesa", post(initiate_mpesa_payment))
        .route("/payment/:id", get(get_payment_status))
        .route("/callback/mpesa", post(mpesa_callback))
}
