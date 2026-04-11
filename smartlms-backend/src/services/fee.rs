// Fee Management Service - Fee structure, payments, M-Pesa integration
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Fee structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeStructure {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub amount: i64, // In smallest currency unit (cents)
    pub currency: String,
    pub fee_type: FeeType,
    pub academic_year: String,
    pub semester: Option<String>,
    pub due_date: DateTime<Utc>,
    pub is_optional: bool,
    pub late_fee_amount: Option<i64>,
    pub late_fee_grace_days: Option<i32>,
}

/// Fee type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeeType {
    Tuition,
    Registration,
    Library,
    Lab,
    Accommodation,
    Examination,
    Certificate,
    Other,
}

/// Student fee record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentFee {
    pub id: uuid::Uuid,
    pub fee_structure_id: uuid::Uuid,
    pub student_id: uuid::Uuid,
    pub amount: i64,
    pub amount_paid: i64,
    pub balance: i64,
    pub status: FeeStatus,
    pub due_date: DateTime<Utc>,
    pub late_fee_applied: i64,
}

/// Fee status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeeStatus {
    Pending,
    Partial,
    Paid,
    Overdue,
    Waived,
}

/// Payment record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: uuid::Uuid,
    pub student_fee_id: uuid::Uuid,
    pub amount: i64,
    pub payment_method: PaymentMethod,
    pub transaction_id: String,
    pub status: PaymentStatus,
    pub gateway_response: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Payment method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    Card,  // Stripe
    Mpesa, // Mobile money (Kenya)
    BankTransfer,
    Cash,
    Cheque,
}

/// Payment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
}

/// M-Pesa configuration
#[derive(Debug, Clone)]
pub struct MpesaConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub short_code: String,
    passkey: String,
    pub environment: MpesaEnvironment,
    pub callback_url: String,
}

/// M-Pesa environment
#[derive(Debug, Clone, Copy)]
pub enum MpesaEnvironment {
    Sandbox,
    Production,
}

/// M-Pesa payment request
#[derive(Debug, Clone, Deserialize)]
pub struct MpesaPaymentRequest {
    pub phone_number: String,
    pub amount: i64,
    pub account_reference: String,
    pub transaction_desc: String,
}

/// M-Pesa STK Push response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpesaStkPushResponse {
    pub merchant_request_id: String,
    pub checkout_request_id: String,
    pub response_code: String,
    pub response_description: String,
    pub customer_message: String,
}

/// M-Pesa callback
#[derive(Debug, Clone, Deserialize)]
pub struct MpesaCallback {
    pub transaction_type: String,
    pub transaction_id: String,
    pub amount: i64,
    pub phone_number: String,
    pub account_reference: String,
    pub status: String,
}

/// M-Pesa callback body
#[derive(Debug, Clone, Deserialize)]
pub struct MpesaCallbackBody {
    pub stk_callback: MpesaStkCallback,
}

/// M-Pesa STK callback
#[derive(Debug, Clone, Deserialize)]
pub struct MpesaStkCallback {
    pub merchant_request_id: String,
    pub checkout_request_id: String,
    pub result_code: i32,
    pub result_desc: String,
    pub callback_metadata: Option<String>,
}

/// M-Pesa access token response
#[derive(Debug, Clone, Deserialize)]
pub struct MpesaTokenResponse {
    pub access_token: String,
    pub expires_in: String,
}

/// M-Pesa service for handling STK Push payments
pub struct MpesaService {
    config: MpesaConfig,
    http_client: reqwest::Client,
}

impl MpesaService {
    pub fn new(config: MpesaConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// Get M-Pesa API base URL based on environment
    fn get_base_url(&self) -> &'static str {
        match self.config.environment {
            MpesaEnvironment::Sandbox => "https://sandbox.safaricom.co.ke",
            MpesaEnvironment::Production => "https://api.safaricom.co.ke",
        }
    }

    /// Get OAuth access token
    pub async fn get_access_token(&self) -> Result<String, String> {
        let url = format!("{}/oauth/v1/generate?grant_type=client_credentials", self.get_base_url());
        
        let response = self.http_client
            .get(&url)
            .basic_auth(&self.config.consumer_key, Some(&self.config.consumer_secret))
            .send()
            .await
            .map_err(|e| format!("Failed to get access token: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Token request failed: {}", response.status()));
        }

        let token_response: MpesaTokenResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;

        Ok(token_response.access_token)
    }

    /// Generate password for STK Push
    fn generate_password(&self) -> String {
        use base64::engine::general_purpose::STANDARD as Base64;
        use base64::Engine;
        
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let data = format!(
            "{}{}{}",
            self.config.short_code,
            self.config.passkey,
            timestamp
        );
        Base64.encode(data.as_bytes())
    }

    /// Initiate STK Push payment
    pub async fn initiate_stk_push(
        &self,
        request: &MpesaPaymentRequest,
    ) -> Result<MpesaStkPushResponse, String> {
        let access_token = self.get_access_token().await?;
        let password = self.generate_password();
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

        // Format phone number (remove leading 0, add 254)
        let formatted_phone = if request.phone_number.starts_with("0") {
            format!("254{}", &request.phone_number[1..])
        } else if request.phone_number.starts_with("+") {
            request.phone_number[1..].to_string()
        } else {
            request.phone_number.clone()
        };

        let stk_request = serde_json::json!({
            "BusinessShortCode": self.config.short_code,
            "Password": password,
            "Timestamp": timestamp,
            "TransactionType": "CustomerPayBillOnline",
            "Amount": request.amount,
            "PartyA": formatted_phone,
            "PartyB": self.config.short_code,
            "PhoneNumber": formatted_phone,
            "CallBackURL": self.config.callback_url,
            "AccountReference": request.account_reference,
            "TransactionDesc": request.transaction_desc,
        });

        let url = format!("{}/mpesa/stkpush/v1/processrequest", self.get_base_url());
        
        let response = self.http_client
            .post(&url)
            .bearer_auth(&access_token)
            .json(&stk_request)
            .send()
            .await
            .map_err(|e| format!("STK Push request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("STK Push failed: {}", response.status()));
        }

        let stk_response: MpesaStkPushResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse STK response: {}", e))?;

        if stk_response.response_code != "0" {
            return Err(stk_response.response_description.clone());
        }

        Ok(stk_response)
    }

    /// Query STK Push payment status
    pub async fn query_stk_status(
        &self,
        checkout_request_id: &str,
    ) -> Result<MpesaStkCallback, String> {
        let access_token = self.get_access_token().await?;
        let password = self.generate_password();
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let query_request = serde_json::json!({
            "BusinessShortCode": self.config.short_code,
            "Password": password,
            "Timestamp": timestamp,
            "CheckoutRequestID": checkout_request_id,
        });

        let url = format!("{}/mpesa/stkpushquery/v1/query", self.get_base_url());
        
        let response = self.http_client
            .post(&url)
            .bearer_auth(&access_token)
            .json(&query_request)
            .send()
            .await
            .map_err(|e| format!("STK query request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("STK query failed: {}", response.status()));
        }

        let callback: MpesaStkCallback = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse query response: {}", e))?;

        Ok(callback)
    }

    /// Process M-Pesa callback from Safaricom
    pub async fn process_callback(
        &self,
        callback_body: MpesaCallbackBody,
    ) -> Result<MpesaPaymentResult, String> {
        let stk_callback = &callback_body.stk_callback;
        
        let result = if stk_callback.result_code == 0 {
            MpesaPaymentResult {
                success: true,
                transaction_id: stk_callback.checkout_request_id.clone(),
                amount: 0, // Would extract from callback metadata
                message: "Payment successful".to_string(),
            }
        } else {
            MpesaPaymentResult {
                success: false,
                transaction_id: stk_callback.checkout_request_id.clone(),
                amount: 0,
                message: stk_callback.result_desc.clone(),
            }
        };

        Ok(result)
    }
}

/// M-Pesa payment result
#[derive(Debug, Clone, Serialize)]
pub struct MpesaPaymentResult {
    pub success: bool,
    pub transaction_id: String,
    pub amount: i64,
    pub message: String,
}

// Service functions
pub mod service {
    use super::*;

    /// Create fee structure for institution
    pub async fn create_fee_structure(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        req: &CreateFeeStructureRequest,
    ) -> Result<FeeStructure, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO fee_structures (id, institution_id, name, description, amount, currency, 
             fee_type, academic_year, semester, due_date, is_optional, late_fee_amount, late_fee_grace_days)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
            id, institution_id, req.name, req.description, req.amount, req.currency,
            format!("{:?}", req.fee_type).to_lowercase(), req.academic_year, req.semester,
            req.due_date, req.is_optional, req.late_fee_amount, req.late_fee_grace_days
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(FeeStructure {
            id,
            institution_id,
            name: req.name.clone(),
            description: req.description.clone(),
            amount: req.amount,
            currency: req.currency.clone(),
            fee_type: req.fee_type,
            academic_year: req.academic_year.clone(),
            semester: req.semester.clone(),
            due_date: req.due_date,
            is_optional: req.is_optional,
            late_fee_amount: req.late_fee_amount,
            late_fee_grace_days: req.late_fee_grace_days,
        })
    }

    /// Assign fee to student
    pub async fn assign_fee_to_student(
        pool: &PgPool,
        fee_structure_id: uuid::Uuid,
        student_id: uuid::Uuid,
    ) -> Result<StudentFee, String> {
        // Get fee structure
        let fee = get_fee_structure(pool, fee_structure_id)
            .await?
            .ok_or("Fee structure not found")?;

        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO student_fees (id, fee_structure_id, student_id, amount, amount_paid, 
             balance, status, due_date, late_fee_applied)
             VALUES ($1, $2, $3, $4, 0, $4, 'pending', $5, 0)",
            id,
            fee_structure_id,
            student_id,
            fee.amount,
            fee.due_date
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(StudentFee {
            id,
            fee_structure_id,
            student_id,
            amount: fee.amount,
            amount_paid: 0,
            balance: fee.amount,
            status: FeeStatus::Pending,
            due_date: fee.due_date,
            late_fee_applied: 0,
        })
    }

    /// Process M-Pesa payment via STK Push
    pub async fn process_mpesa_payment(
        pool: &PgPool,
        mpesa_service: &MpesaService,
        student_fee_id: uuid::Uuid,
        req: &MpesaPaymentRequest,
    ) -> Result<Payment, String> {
        // Create pending payment record
        let payment_id = Uuid::new_v4();
        let transaction_id = format!("MPESA{}", Uuid::new_v4().to_string()[..8].to_uppercase());
        
        let payment = Payment {
            id: payment_id,
            student_fee_id,
            amount: req.amount,
            payment_method: PaymentMethod::Mpesa,
            transaction_id: transaction_id.clone(),
            status: PaymentStatus::Processing,
            gateway_response: None,
            paid_at: None,
            created_at: Utc::now(),
        };

        // Save initial payment record
        sqlx::query!(
            "INSERT INTO payments (id, student_fee_id, amount, payment_method, transaction_id, 
             status, paid_at, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            payment.id,
            payment.student_fee_id,
            payment.amount,
            "mpesa",
            payment.transaction_id,
            "processing",
            payment.paid_at,
            payment.created_at
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Initiate STK Push
        match mpesa_service.initiate_stk_push(req).await {
            Ok(stk_response) => {
                // Update payment with checkout request ID
                sqlx::query!(
                    "UPDATE payments SET gateway_response = $1 WHERE id = $2",
                    serde_json::to_string(&stk_response).unwrap_or_default(),
                    payment_id
                )
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

                Ok(Payment {
                    gateway_response: Some(serde_json::to_string(&stk_response).unwrap_or_default()),
                    ..payment
                })
            }
            Err(e) => {
                // Mark payment as failed
                sqlx::query!(
                    "UPDATE payments SET status = 'failed' WHERE id = $1",
                    payment_id
                )
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

                Err(format!("M-Pesa STK Push failed: {}", e))
            }
        }
    }

    /// Process M-Pesa callback and update payment status
    pub async fn process_mpesa_callback(
        pool: &PgPool,
        callback: &MpesaStkCallback,
    ) -> Result<(), String> {
        // Find payment by checkout request ID
        let payment = sqlx::query_as!(
            Payment,
            "SELECT * FROM payments WHERE gateway_response LIKE $1",
            format!("%{}%", callback.checkout_request_id)
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Payment not found for callback")?;

        let new_status = if callback.result_code == 0 {
            PaymentStatus::Completed
        } else {
            PaymentStatus::Failed
        };

        // Update payment status
        sqlx::query!(
            "UPDATE payments SET status = $1, paid_at = $2 WHERE id = $3",
            format!("{:?}", new_status).to_lowercase(),
            if new_status == PaymentStatus::Completed { Some(Utc::now()) } else { None },
            payment.id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // If successful, update student fee balance
        if new_status == PaymentStatus::Completed {
            sqlx::query!(
                "UPDATE student_fees SET 
                    amount_paid = amount_paid + $1,
                    balance = amount - (amount_paid + $1),
                    status = CASE 
                        WHEN amount - (amount_paid + $1) <= 0 THEN 'paid' 
                        ELSE 'partial' 
                    END
                 WHERE id = $2",
                payment.amount,
                payment.student_fee_id
            )
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Process Stripe card payment
    pub async fn process_card_payment(
        pool: &PgPool,
        student_fee_id: uuid::Uuid,
        amount: i64,
        payment_intent_id: &str,
    ) -> Result<Payment, String> {
        // In production: verify with Stripe API

        let payment = Payment {
            id: Uuid::new_v4(),
            student_fee_id,
            amount,
            payment_method: PaymentMethod::Card,
            transaction_id: payment_intent_id.to_string(),
            status: PaymentStatus::Completed,
            gateway_response: None,
            paid_at: Some(Utc::now()),
            created_at: Utc::now(),
        };

        // Save payment
        sqlx::query!(
            "INSERT INTO payments (id, student_fee_id, amount, payment_method, transaction_id, 
             status, paid_at, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            payment.id,
            payment.student_fee_id,
            payment.amount,
            "card",
            payment.transaction_id,
            "completed",
            payment.paid_at,
            payment.created_at
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Update student fee balance
        sqlx::query!(
            "UPDATE student_fees SET 
                amount_paid = amount_paid + $1,
                balance = amount - (amount_paid + $1),
                status = CASE 
                    WHEN amount - (amount_paid + $1) <= 0 THEN 'paid' 
                    ELSE 'partial' 
                END
             WHERE id = $2",
            amount,
            student_fee_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(payment)
    }

    /// Get student fee summary
    pub async fn get_student_fee_summary(
        pool: &PgPool,
        student_id: uuid::Uuid,
    ) -> Result<FeeSummary, String> {
        let row = sqlx::query!(
            "SELECT 
                SUM(amount) as total_fees,
                SUM(amount_paid) as total_paid,
                SUM(balance) as total_balance,
                COUNT(CASE WHEN status = 'overdue' THEN 1 END) as overdue_count
             FROM student_fees WHERE student_id = $1",
            student_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(FeeSummary {
            total_fees: row.total_fees.unwrap_or(0),
            total_paid: row.total_paid.unwrap_or(0),
            total_balance: row.total_balance.unwrap_or(0),
            overdue_fees: row.overdue_count.unwrap_or(0) as i32,
        })
    }

    /// Generate invoice
    pub async fn generate_invoice(
        pool: &PgPool,
        student_id: uuid::Uuid,
    ) -> Result<Invoice, String> {
        let rows = sqlx::query!(
            "SELECT sf.id, fs.name, fs.amount, sf.amount_paid, sf.balance, sf.due_date, fs.description
             FROM student_fees sf
             JOIN fee_structures fs ON sf.fee_structure_id = fs.id
             WHERE sf.student_id = $1",
            student_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let invoice_items: Vec<InvoiceItem> = rows
            .into_iter()
            .map(|r| InvoiceItem {
                fee_name: r.name,
                description: r.description,
                amount: r.amount,
                amount_paid: r.amount_paid,
                balance: r.balance,
                due_date: r.due_date,
            })
            .collect();

        let total: i64 = invoice_items.iter().map(|i| i.amount).sum();
        let paid: i64 = invoice_items.iter().map(|i| i.amount_paid).sum();

        Ok(Invoice {
            invoice_number: format!("INV-{}", Utc::now().format("%Y%m%d")),
            student_id,
            items: invoice_items,
            total_amount: total,
            total_paid: paid,
            total_balance: total - paid,
            generated_at: Utc::now(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateFeeStructureRequest {
    pub name: String,
    pub description: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub fee_type: FeeType,
    pub academic_year: String,
    pub semester: Option<String>,
    pub due_date: DateTime<Utc>,
    pub is_optional: bool,
    pub late_fee_amount: Option<i64>,
    pub late_fee_grace_days: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FeeSummary {
    pub total_fees: i64,
    pub total_paid: i64,
    pub total_balance: i64,
    pub overdue_fees: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Invoice {
    pub invoice_number: String,
    pub student_id: uuid::Uuid,
    pub items: Vec<InvoiceItem>,
    pub total_amount: i64,
    pub total_paid: i64,
    pub total_balance: i64,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InvoiceItem {
    pub fee_name: String,
    pub description: Option<String>,
    pub amount: i64,
    pub amount_paid: i64,
    pub balance: i64,
    pub due_date: DateTime<Utc>,
}
