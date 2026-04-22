// Bulk User Import Service - CSV import for users
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// CSV row format for user import
#[derive(Debug, Clone, Deserialize)]
pub struct UserCsvRow {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub department: Option<String>,
    pub student_id: Option<String>,
    pub phone: Option<String>,
}

/// Import result
#[derive(Debug, Clone, Serialize)]
pub struct ImportResult {
    pub total_rows: i32,
    pub successful: i32,
    pub failed: i32,
    pub errors: Vec<ImportError>,
    pub created_users: Vec<CreatedUser>,
    pub skipped_users: Vec<SkippedUser>,
}

/// Single import error
#[derive(Debug, Clone, Serialize)]
pub struct ImportError {
    pub row: i32,
    pub field: String,
    pub message: String,
}

/// Successfully created user
#[derive(Debug, Clone, Serialize)]
pub struct CreatedUser {
    pub row: i32,
    pub user_id: Uuid,
    pub email: String,
}

/// Skipped (already exists) user
#[derive(Debug, Clone, Serialize)]
pub struct SkippedUser {
    pub row: i32,
    pub email: String,
    pub reason: String,
}

/// Service functions
pub mod service {
    use super::*;
    use std::io::Cursor;

    /// Import users from CSV data
    pub async fn import_users_from_csv(
        pool: &PgPool,
        csv_data: &str,
        default_role: &str,
    ) -> Result<ImportResult, String> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(Cursor::new(csv_data));

        let mut result = ImportResult {
            total_rows: 0,
            successful: 0,
            failed: 0,
            errors: vec![],
            created_users: vec![],
            skipped_users: vec![],
        };

        for (idx, row_result) in reader.records().enumerate() {
            let row_num = (idx + 2) as i32; // +2 because header is row 1
            result.total_rows += 1;

            match row_result {
                Ok(record) => {
                    // Parse the row
                    let email = record.get(0).unwrap_or("").trim().to_string();
                    let first_name = record.get(1).unwrap_or("").trim().to_string();
                    let last_name = record.get(2).unwrap_or("").trim().to_string();
                    let role = record.get(3).unwrap_or(default_role).trim().to_string();
                    let department = record
                        .get(4)
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty());
                    let student_id = record
                        .get(5)
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty());
                    let phone = record
                        .get(6)
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty());

                    // Validate required fields
                    if email.is_empty() {
                        result.failed += 1;
                        result.errors.push(ImportError {
                            row: row_num,
                            field: "email".to_string(),
                            message: "Email is required".to_string(),
                        });
                        continue;
                    }

                    if !email.contains('@') {
                        result.failed += 1;
                        result.errors.push(ImportError {
                            row: row_num,
                            field: "email".to_string(),
                            message: "Invalid email format".to_string(),
                        });
                        continue;
                    }

                    if first_name.is_empty() || last_name.is_empty() {
                        result.failed += 1;
                        result.errors.push(ImportError {
                            row: row_num,
                            field: "name".to_string(),
                            message: "First and last name are required".to_string(),
                        });
                        continue;
                    }

                    // Check if user already exists
                    if let Ok(Some(_)) = crate::db::user::find_by_email(pool, &email).await {
                        result.skipped_users.push(SkippedUser {
                            row: row_num,
                            email: email.clone(),
                            reason: "User already exists".to_string(),
                        });
                        continue;
                    }

                    // Create the user
                    match crate::db::user::create(
                        pool,
                        &email,
                        "", // No password - must be set by user
                        &first_name,
                        &last_name,
                        &role,
                    )
                    .await
                    {
                        Ok(user) => {
                            result.successful += 1;
                            result.created_users.push(CreatedUser {
                                row: row_num,
                                user_id: user.id,
                                email: user.email,
                            });

                            // TODO: Update extended profile (department, student_id, phone)
                            // This would require additional DB tables
                        }
                        Err(e) => {
                            result.failed += 1;
                            result.errors.push(ImportError {
                                row: row_num,
                                field: "create".to_string(),
                                message: e.to_string(),
                            });
                        }
                    }
                }
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(ImportError {
                        row: row_num,
                        field: "parse".to_string(),
                        message: format!("CSV parse error: {}", e),
                    });
                }
            }
        }

        // Log import event
        tracing::info!(
            "Bulk import completed: {} total, {} created, {} failed, {} skipped",
            result.total_rows,
            result.successful,
            result.failed,
            result.skipped_users.len()
        );

        Ok(result)
    }

    /// Generate sample CSV template
    pub fn generate_csv_template() -> String {
        "email,first_name,last_name,role,department,student_id,phone
admin@example.com,John,Admin,admin,Administration,,
instructor@example.com,Jane,Instructor,instructor,Computer Science,,
student@example.com,Bob,Student,learner,Computer Science,S12345,+1234567890"
            .to_string()
    }

    /// Validate CSV before import
    pub fn validate_csv(csv_data: &str) -> Result<Vec<String>, String> {
        let mut errors = Vec::new();

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(csv_data));

        // Check headers
        let headers = reader.headers().map_err(|e| e.to_string())?;
        let expected_headers = vec!["email", "first_name", "last_name", "role"];

        for expected in expected_headers {
            if !headers.iter().any(|h| h.to_lowercase() == expected) {
                errors.push(format!("Missing required header: {}", expected));
            }
        }

        // Check row count (max 1000)
        let row_count = reader.records().count();
        if row_count > 1000 {
            errors.push(format!("Maximum 1000 rows allowed, got {}", row_count));
        }

        Ok(errors)
    }
}
