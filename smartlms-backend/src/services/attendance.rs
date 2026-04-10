// Standalone Attendance System - QR codes, manual entry, reports
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Attendance type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttendanceType {
    QRCode,       // QR scan
    Manual,       // Manual entry by instructor
    Auto,         // Auto-check based on activity
    SelfCheck,    // Student self-check-in
}

/// Attendance record for a class session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassAttendance {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,         // For scheduled classes
    pub course_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub attendance_type: AttendanceType,
    pub status: AttendanceStatus,
    pub marked_at: DateTime<Utc>,
    pub marked_by: Option<uuid::Uuid>,  // Who marked (instructor or self)
    pub location: Option<String>,       // For location-based check-in
    pub device_info: Option<String>,    // Device used for check-in
    pub notes: Option<String>,
}

/// QR Code session for attendance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRCodeSession {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub code: String,                   // The actual QR code value
    pub code_url: Option<String>,       // Generated QR image URL
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub max_uses: Option<i32>,          // Max times it can be scanned
    pub used_count: i32,
    pub location_radius_meters: Option<i32>, // For geo-fencing
}

/// QR Check-in request
#[derive(Debug, Deserialize)]
pub struct QRCheckInRequest {
    pub code: String,
    pub user_id: uuid::Uuid,
    pub location: Option<String>,
    pub device_info: Option<String>,
}

/// Manual attendance request
#[derive(Debug, Deserialize)]
pub struct ManualAttendanceRequest {
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub session_id: Option<uuid::Uuid>,
    pub date: Option<DateTime<Utc>>,
    pub status: AttendanceStatus,
    pub notes: Option<String>,
}

/// Bulk manual attendance
#[derive(Debug, Deserialize)]
pub struct BulkAttendanceRequest {
    pub attendances: Vec<ManualAttendanceRequest>,
    pub marked_by: uuid::Uuid,
}

/// Attendance summary for a course/date range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceReport {
    pub course_id: uuid::Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_sessions: i32,
    pub total_students: i32,
    pub present_count: i32,
    pub late_count: i32,
    pub absent_count: i32,
    pub excused_count: i32,
    pub overall_attendance_rate: f32,
    pub student_details: Vec<StudentAttendanceRecord>,
}

/// Individual student attendance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentAttendanceRecord {
    pub user_id: uuid::Uuid,
    pub student_name: String,
    pub present: i32,
    pub late: i32,
    pub absent: i32,
    pub excused: i32,
    pub attendance_rate: f32,
}

/// Daily attendance sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyAttendanceSheet {
    pub date: DateTime<Utc>,
    pub course_id: uuid::Uuid,
    pub sessions: Vec<SessionAttendanceSummary>,
    pub students: Vec<StudentDailyAttendance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAttendanceSummary {
    pub session_id: uuid::Uuid,
    pub session_time: String,
    pub present: i32,
    pub absent: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentDailyAttendance {
    pub user_id: uuid::Uuid,
    pub student_name: String,
    pub status_by_session: std::collections::HashMap<uuid::Uuid, AttendanceStatus>,
}

// Service functions
pub mod service {
    use super::*;
    use crate::db::attendance as attendance_db;
    use sqlx::PgPool;
    use uuid::Uuid;
    
    /// Generate a new QR code for a session
    pub async fn generate_qr_code(
        pool: &PgPool,
        session_id: uuid::Uuid,
        expires_minutes: i64,
        max_uses: Option<i32>,
    ) -> Result<QRCodeSession, String> {
        // Generate random code
        let code = generate_random_code(6);
        let expires = Utc::now() + chrono::Duration::minutes(expires_minutes);
        
        let session = attendance_db::create_qr_session(
            pool,
            session_id,
            &code,
            expires,
            max_uses,
        ).await.map_err(|e| e.to_string())?;
        
        Ok(session)
    }
    
    fn generate_random_code(length: usize) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let code: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 { (b'0' + idx) as char }
                else { (b'A' + idx - 10) as char }
            })
            .collect();
        code
    }
    
    /// Validate and use QR code for check-in
    pub async fn check_in_with_qr(
        pool: &PgPool,
        req: &QRCheckInRequest,
    ) -> Result<ClassAttendance, String> {
        // Find the QR session
        let qr_session = attendance_db::get_active_qr_by_code(pool, &req.code).await
            .map_err(|e| e.to_string())?
            .ok_or("Invalid or expired QR code".to_string())?;
        
        // Check if expired
        if Utc::now() > qr_session.expires_at {
            return Err("QR code has expired".to_string());
        }
        
        // Check max uses
        if let Some(max) = qr_session.max_uses {
            if qr_session.used_count >= max {
                return Err("QR code max uses reached".to_string());
            }
        }
        
        // Create attendance record
        let attendance = ClassAttendance {
            id: Uuid::new_v4(),
            session_id: qr_session.session_id,
            course_id: uuid::Uuid::nil(), // Get from session
            user_id: req.user_id,
            attendance_type: AttendanceType::QRCode,
            status: AttendanceStatus::Present,
            marked_at: Utc::now(),
            marked_by: Some(req.user_id), // Self check-in
            location: req.location.clone(),
            device_info: req.device_info.clone(),
            notes: None,
        };
        
        // Save attendance and increment QR use count
        attendance_db::create_class_attendance(pool, &attendance).await
            .map_err(|e| e.to_string())?;
        
        attendance_db::increment_qr_use_count(pool, qr_session.id).await
            .map_err(|e| e.to_string())?;
        
        Ok(attendance)
    }
    
    /// Mark manual attendance
    pub async fn mark_manual_attendance(
        pool: &PgPool,
        req: &ManualAttendanceRequest,
        marker_id: uuid::Uuid,
    ) -> Result<ClassAttendance, String> {
        let attendance = ClassAttendance {
            id: Uuid::new_v4(),
            session_id: req.session_id.unwrap_or(uuid::Uuid::nil()),
            course_id: req.course_id,
            user_id: req.user_id,
            attendance_type: AttendanceType::Manual,
            status: req.status,
            marked_at: Utc::now(),
            marked_by: Some(marker_id),
            location: None,
            device_info: None,
            notes: req.notes.clone(),
        };
        
        attendance_db::create_class_attendance(pool, &attendance).await
            .map_err(|e| e.to_string())
    }
    
    /// Bulk mark attendance
    pub async fn bulk_mark_attendance(
        pool: &PgPool,
        req: &BulkAttendanceRequest,
    ) -> Result<Vec<ClassAttendance>, String> {
        let mut results = Vec::new();
        
        for att in &req.attendances {
            let result = mark_manual_attendance(
                pool,
                &ManualAttendanceRequest {
                    user_id: att.user_id,
                    course_id: att.course_id,
                    session_id: att.session_id,
                    date: att.date,
                    status: att.status,
                    notes: att.notes.clone(),
                },
                req.marked_by,
            ).await?;
            
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Generate attendance report for a course
    pub async fn get_attendance_report(
        pool: &PgPool,
        course_id: uuid::Uuid,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<AttendanceReport, String> {
        let records = attendance_db::get_course_attendance(
            pool,
            course_id,
            start_date,
            end_date,
        ).await.map_err(|e| e.to_string())?;
        
        // Group by user
        let mut student_stats: std::collections::HashMap<uuid::Uuid, (String, i32, i32, i32, i32)> = 
            std::collections::HashMap::new();
        
        for record in &records {
            let stats = student_stats.entry(record.user_id).or_insert((String::new(), 0, 0, 0, 0));
            stats.0 = format!("Student {}", &record.user_id.to_string()[..8]); // Simplified
            
            match record.status {
                AttendanceStatus::Present => stats.1 += 1,
                AttendanceStatus::Late => stats.2 += 1,
                AttendanceStatus::Absent => stats.3 += 1,
                AttendanceStatus::Excused => stats.4 += 1,
            }
        }
        
        let total_students = student_stats.len() as i32;
        let total_sessions = records.len() as i32;
        let present = student_stats.values().map(|s| s.1).sum::<i32>();
        let late = student_stats.values().map(|s| s.2).sum::<i32>();
        let absent = student_stats.values().map(|s| s.3).sum::<i32>();
        let excused = student_stats.values().map(|s| s.4).sum::<i32>();
        
        let rate = if total_students > 0 && total_sessions > 0 {
            ((present + late) as f32 / (total_students * total_sessions) as f32) * 100.0
        } else {
            0.0
        };
        
        let student_details: Vec<StudentAttendanceRecord> = student_stats
            .into_iter()
            .map(|(user_id, (name, present, late, absent, excused))| {
                let total = present + late + absent + excused;
                let rate = if total > 0 {
                    ((present + late) as f32 / total as f32) * 100.0
                } else {
                    0.0
                };
                
                StudentAttendanceRecord {
                    user_id,
                    student_name: name,
                    present,
                    late,
                    absent,
                    excused,
                    attendance_rate: rate,
                }
            })
            .collect();
        
        Ok(AttendanceReport {
            course_id,
            start_date,
            end_date,
            total_sessions,
            total_students,
            present_count: present,
            late_count: late,
            absent_count: absent,
            excused_count: excused,
            overall_attendance_rate: rate,
            student_details,
        })
    }
    
    /// Export attendance to CSV format
    pub async fn export_attendance_csv(
        pool: &PgPool,
        course_id: uuid::Uuid,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<String, String> {
        let report = get_attendance_report(pool, course_id, start_date, end_date).await?;
        
        let mut csv = String::from("Student ID,Name,Present,Late,Absent,Excused,Attendance Rate\n");
        
        for student in &report.student_details {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{:.1}%\n",
                student.user_id,
                student.student_name,
                student.present,
                student.late,
                student.absent,
                student.excused,
                student.attendance_rate
            ));
        }
        
        Ok(csv)
    }
}