// Exam Cards API — Generate and manage exam admission cards
use crate::models::parents_alumni::*;
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::{get, post},
    Router,
};
use chrono::{DateTime, NaiveTime, Utc};
use printpdf::{
    IndirectFontRef, Line, Mm, PdfDocument, Point, Pt,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// ============================================
// DATA STRUCTURES
// ============================================

/// Exam entry in an exam card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamCardEntry {
    pub course: String,
    pub code: String,
    pub date: DateTime<Utc>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub venue: String,
    pub seat_number: String,
    pub duration_minutes: i32,
    pub exam_type: String,
}

/// Student information for exam card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamCardStudent {
    pub student_id: Uuid,
    pub name: String,
    pub reg_number: String,
    pub programme: String,
    pub year_of_study: String,
    pub academic_year: String,
    pub photo_url: Option<String>,
    pub fee_status: String, // "cleared", "pending", "blocked"
}

/// Full exam card response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamCard {
    pub card_id: Uuid,
    pub card_number: String,
    pub student: ExamCardStudent,
    pub exams: Vec<ExamCardEntry>,
    pub issued_at: DateTime<Utc>,
    pub qr_code_data: String,
    pub is_valid: bool,
}

/// Request to generate exam card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateExamCardRequest {
    pub student_id: Uuid,
    pub academic_year: String,
    pub semester: u8,
}

/// Exam card list response (for admin)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamCardListResponse {
    pub cards: Vec<ExamCardSummary>,
    pub total: i64,
    pub cleared_count: i64,
    pub pending_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamCardSummary {
    pub card_id: Uuid,
    pub student_id: Uuid,
    pub student_name: String,
    pub reg_number: String,
    pub status: String,
    pub issued_at: DateTime<Utc>,
}

// ============================================
// SERVICE IMPLEMENTATION
// ============================================

pub struct ExamCardsService;

impl ExamCardsService {
    /// Generate exam card for a student
    pub async fn generate_exam_card(
        pool: &PgPool,
        req: GenerateExamCardRequest,
    ) -> Result<ExamCard, sqlx::Error> {
        let card_id = Uuid::new_v4();
        let card_number = format!("EC-{}-{:06}", req.academic_year.replace("/", ""), rand::random::<u32>());
        let now = Utc::now();

        // TODO: Fetch student data from users table
        // TODO: Check fee clearance status
        // TODO: Fetch enrolled courses and exam schedule
        
        // Placeholder data
        let student = ExamCardStudent {
            student_id: req.student_id,
            name: "Student Name".to_string(),
            reg_number: "REG/2024/001".to_string(),
            programme: "BSc Computer Science".to_string(),
            year_of_study: "Year 3".to_string(),
            academic_year: req.academic_year,
            photo_url: None,
            fee_status: "cleared".to_string(),
        };

        // Placeholder exam entries
        let exams = vec![
            ExamCardEntry {
                course: "Data Structures & Algorithms".to_string(),
                code: "CS301".to_string(),
                date: now + chrono::Duration::days(30),
                start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                end_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                venue: "Main Hall A".to_string(),
                seat_number: "A-24".to_string(),
                duration_minutes: 180,
                exam_type: "End of Semester".to_string(),
            },
        ];

        let qr_code_data = format!("SLMS-EC-{}-{}", card_id, student.reg_number);
        let is_valid = student.fee_status == "cleared";

        let card = ExamCard {
            card_id,
            card_number,
            student,
            exams,
            issued_at: now,
            qr_code_data,
            is_valid,
        };

        // TODO: Save to database
        Ok(card)
    }

    /// Get student's exam card
    pub async fn get_student_exam_card(
        pool: &PgPool,
        student_id: Uuid,
    ) -> Result<Option<ExamCard>, sqlx::Error> {
        let _ = (pool, student_id);
        // TODO: SELECT FROM exam_cards WHERE student_id = $1
        Ok(None)
    }

    /// Get all exam cards (admin view)
    pub async fn get_all_exam_cards(
        pool: &PgPool,
        institution_id: Uuid,
        academic_year: &str,
    ) -> Result<ExamCardListResponse, sqlx::Error> {
        let _ = (pool, institution_id, academic_year);
        // TODO: SELECT FROM exam_cards WITH student details
        Ok(ExamCardListResponse {
            cards: vec![],
            total: 0,
            cleared_count: 0,
            pending_count: 0,
        })
    }

    /// Generate PDF for exam card
    pub fn generate_exam_card_pdf(card: &ExamCard) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use printpdf::{PdfLayerIndex, PdfPageIndex, Rgb};
        
        // Create PDF document
        let (mut doc, page1, layer1) = PdfDocument::new(
            "Exam Admission Card",
            Mm(210.0), // A4 width
            Mm(148.0), // A5 height
            "Page 1",
        );
        
        let font = doc.add_builtin_font(printpdf::BuiltinFont::Helvetica)?;
        let font_bold = doc.add_builtin_font(printpdf::BuiltinFont::HelveticaBold)?;
        
        // Add content to layer
        Self::add_pdf_content(&mut doc, page1, layer1, card, &font, &font_bold)?;
        
        // Write to buffer
        let mut buffer = Vec::new();
        doc.save_to(&mut buffer)?;
        
        Ok(buffer)
    }
    
    fn add_pdf_content(
        doc: &mut PdfDocument,
        page: PdfPageIndex,
        layer: PdfLayerIndex,
        card: &ExamCard,
        font: &IndirectFontRef,
        font_bold: &IndirectFontRef,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use printpdf::{LineDashPattern, Outline};
        
        let current_layer = doc.get_page(page).get_layer(layer);
        
        // Header background (brand color simulation with gray)
        let header_rect = Line {
            points: vec![
                Point::new(Mm(0.0), Mm(0.0)),
                Point::new(Mm(210.0), Mm(0.0)),
                Point::new(Mm(210.0), Mm(30.0)),
                Point::new(Mm(0.0), Mm(30.0)),
            ],
            is_closed: true,
            has_fill: true,
            has_stroke: false,
            is_clipping_path: false,
        };
        current_layer.add_shape(header_rect, &printpdf::Color::Rgb(Rgb::new(0.2, 0.4, 0.6, None)));
        
        // Title
        current_layer.use_text(
            "SmartLMS University",
            font_bold,
            18.0,
            Mm(15.0),
            Mm(130.0),
        );
        
        current_layer.use_text(
            "Examination Admission Card",
            font,
            12.0,
            Mm(15.0),
            Mm(115.0),
        );
        
        // Student info
        let mut y_pos = Mm(95.0);
        current_layer.use_text(
            &format!("Name: {}", card.student.name),
            font,
            11.0,
            Mm(15.0),
            y_pos,
        );
        y_pos -= Mm(8.0);
        
        current_layer.use_text(
            &format!("Registration No: {}", card.student.reg_number),
            font,
            11.0,
            Mm(15.0),
            y_pos,
        );
        y_pos -= Mm(8.0);
        
        current_layer.use_text(
            &format!("Programme: {}", card.student.programme),
            font,
            11.0,
            Mm(15.0),
            y_pos,
        );
        y_pos -= Mm(8.0);
        
        current_layer.use_text(
            &format!("Academic Year: {}", card.student.academic_year),
            font,
            11.0,
            Mm(15.0),
            y_pos,
        );
        y_pos -= Mm(12.0);
        
        // Exam schedule header
        current_layer.use_text(
            "Examination Schedule",
            font_bold,
            12.0,
            Mm(15.0),
            y_pos,
        );
        y_pos -= Mm(8.0);
        
        // List exams
        for exam in &card.exams {
            let exam_text = format!(
                "{} - {} | {} {} to {} | {} | Seat: {}",
                exam.code,
                exam.course,
                exam.date.format("%Y-%m-%d"),
                exam.start_time.format("%H:%M"),
                exam.end_time.format("%H:%M"),
                exam.venue,
                exam.seat_number
            );
            
            current_layer.use_text(
                &exam_text,
                font,
                9.0,
                Mm(15.0),
                y_pos,
            );
            y_pos -= Mm(7.0);
        }
        
        // Footer
        y_pos -= Mm(5.0);
        current_layer.use_text(
            &format!("Card No: {}", card.card_number),
            font,
            10.0,
            Mm(15.0),
            y_pos,
        );
        y_pos -= Mm(6.0);
        
        current_layer.use_text(
            "This card must be presented at each examination. Tampering is a disciplinary offence.",
            font,
            8.0,
            Mm(15.0),
            y_pos,
        );
        
        Ok(())
    }
}

// ============================================
// API HANDLERS
// ============================================

/// Generate exam card for student
pub async fn generate_exam_card_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<GenerateExamCardRequest>,
) -> Result<Json<ExamCard>, (StatusCode, String)> {
    // Verify permissions
    if user.role != "admin" && user.role != "exams_officer" {
        return Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()));
    }
    
    let card = ExamCardsService::generate_exam_card(&pool, req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(card))
}

/// Get my exam card (student endpoint)
pub async fn get_my_exam_card_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
) -> Result<Json<ExamCard>, (StatusCode, String)> {
    let card = ExamCardsService::get_student_exam_card(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Exam card not found".to_string()))?;
    
    Ok(Json(card))
}

/// Download exam card as PDF
pub async fn download_exam_card_pdf_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
) -> Result<(HeaderMap, Vec<u8>), (StatusCode, String)> {
    let card = ExamCardsService::get_student_exam_card(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Exam card not found".to_string()))?;
    
    let pdf_bytes = ExamCardsService::generate_exam_card_pdf(&card)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("PDF generation failed: {}", e)))?;
    
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/pdf"),
    );
    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str(&format!("attachment; filename=\"exam_card_{}.pdf\"", card.card_number)).unwrap(),
    );
    
    Ok((headers, pdf_bytes))
}

/// Get all exam cards (admin view)
pub async fn get_all_exam_cards_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Query(params): Query<ExamCardListParams>,
) -> Result<Json<ExamCardListResponse>, (StatusCode, String)> {
    if user.role != "admin" && user.role != "exams_officer" {
        return Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()));
    }
    
    let result = ExamCardsService::get_all_exam_cards(&pool, params.institution_id, &params.academic_year)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct ExamCardListParams {
    pub institution_id: Uuid,
    pub academic_year: String,
}

// ============================================
// ROUTER CREATION
// ============================================

pub fn exam_cards_router() -> Router {
    Router::new()
        .route("/my-card", get(get_my_exam_card_handler))
        .route("/download-pdf", get(download_exam_card_pdf_handler))
        .route("/generate", post(generate_exam_card_handler))
        .route("/list", get(get_all_exam_cards_handler))
}
