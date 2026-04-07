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

/// M-Pesa payment request
#[derive(Debug, Clone, Deserialize)]
pub struct MpesaPaymentRequest {
    pub phone_number: String,
    pub amount: i64,
    pub account_reference: String,
    pub transaction_desc: String,
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

    /// Process M-Pesa payment
    pub async fn process_mpesa_payment(
        pool: &PgPool,
        req: &MpesaPaymentRequest,
    ) -> Result<Payment, String> {
        // In production: call M-Pesa API (STK Push)
        // For now, simulate

        let transaction_id = format!("MPESA{}", Uuid::new_v4().to_string()[..8].to_uppercase());

        Ok(Payment {
            id: Uuid::new_v4(),
            student_fee_id: Uuid::nil(), // Would look up by account_reference
            amount: req.amount,
            payment_method: PaymentMethod::Mpesa,
            transaction_id,
            status: PaymentStatus::Completed,
            gateway_response: None,
            paid_at: Some(Utc::now()),
            created_at: Utc::now(),
        })
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
