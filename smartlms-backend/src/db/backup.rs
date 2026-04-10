// Database operations for backup system
use super::models::backup::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub async fn create_backup_record(pool: &PgPool, backup: &Backup) -> Result<Backup, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO backups (id, institution_id, backup_type, status, started_at, retention_days, created_by)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        backup.id, backup.institution_id, format!("{:?}", backup.backup_type).to_lowercase(),
        format!("{:?}", backup.status).to_lowercase(), backup.started_at, 
        backup.retention_days, backup.created_by
    )
    .execute(pool)
    .await?;

    Ok(backup.clone())
}

pub async fn get_backup(pool: &PgPool, id: Uuid) -> Result<Option<Backup>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, institution_id, backup_type, status, file_path, file_size_bytes, 
                started_at, completed_at, checksum, retention_days, created_by
         FROM backups WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Backup {
        id: r.id,
        institution_id: r.institution_id,
        backup_type: BackupType::Full,
        status: BackupStatus::Pending,
        file_path: r.file_path,
        file_size_bytes: r.file_size_bytes,
        started_at: r.started_at,
        completed_at: r.completed_at,
        checksum: r.checksum,
        retention_days: r.retention_days,
        created_by: r.created_by,
    }))
}

pub async fn update_backup_status(pool: &PgPool, id: Uuid, status: BackupStatus) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE backups SET status = $1 WHERE id = $2", format!("{:?}", status).to_lowercase(), id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_backup_completed(
    pool: &PgPool,
    id: Uuid,
    file_path: &str,
    file_size: i64,
    checksum: &str,
) -> Result<Backup, sqlx::Error> {
    let now = Utc::now();
    
    sqlx::query!(
        "UPDATE backups SET status = 'completed', file_path = $1, file_size_bytes = $2, 
         checksum = $3, completed_at = $4 WHERE id = $5",
        file_path, file_size, checksum, now, id
    )
    .execute(pool)
    .await?;

    get_backup(pool, id).await.map(|o| o.unwrap())
}

pub async fn list_backups(pool: &PgPool, institution_id: Uuid, limit: i64) -> Result<Vec<Backup>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, institution_id, backup_type, status, file_path, file_size_bytes, 
                started_at, completed_at, checksum, retention_days, created_by
         FROM backups WHERE institution_id = $1 ORDER BY started_at DESC LIMIT $2",
        institution_id, limit
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Backup {
        id: r.id,
        institution_id: r.institution_id,
        backup_type: BackupType::Full,
        status: BackupStatus::Pending,
        file_path: r.file_path,
        file_size_bytes: r.file_size_bytes,
        started_at: r.started_at,
        completed_at: r.completed_at,
        checksum: r.checksum,
        retention_days: r.retention_days,
        created_by: r.created_by,
    }).collect())
}

pub async fn get_recent_backups(pool: &PgPool, institution_id: Uuid, limit: i64) -> Result<Vec<Backup>, sqlx::Error> {
    list_backups(pool, institution_id, limit).await
}

pub async fn delete_expired_backups(pool: &PgPool, institution_id: Uuid) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM backups WHERE institution_id = $1 AND 
         completed_at < NOW() - INTERVAL '30 days'",
        institution_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}

pub async fn save_verification(pool: &PgPool, backup_id: Uuid, result: &VerificationResult) -> Result<(), sqlx::Error> {
    let table_counts_json = serde_json::to_string(&result.table_counts).unwrap_or_default();
    
    sqlx::query!(
        "INSERT INTO backup_verifications (backup_id, is_valid, schema_valid, data_integrity, 
         table_counts, errors, verified_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        backup_id, result.is_valid, result.schema_valid, result.data_integrity,
        table_counts_json, result.errors.join(","), result.verified_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn log_restore(pool: &PgPool, backup_id: Uuid, result: &RestoreResult) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO backup_restores (backup_id, success, restored_tables, errors, warnings)
         VALUES ($1, $2, $3, $4, $5)",
        backup_id, result.success, result.restored_tables.join(","),
        result.errors.join(","), result.warnings.join(",")
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_schedule(pool: &PgPool, schedule: &BackupSchedule) -> Result<BackupSchedule, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO backup_schedules (id, institution_id, schedule_type, backup_type, 
         interval_hours, retention_days, is_active, next_run_at, last_run_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        schedule.id, schedule.institution_id, format!("{:?}", schedule.schedule_type).to_lowercase(),
        format!("{:?}", schedule.backup_type).to_lowercase(), schedule.interval_hours,
        schedule.retention_days, schedule.is_active, schedule.next_run_at, schedule.last_run_at
    )
    .execute(pool)
    .await?;

    Ok(schedule.clone())
}

pub async fn get_active_schedule(pool: &PgPool, institution_id: Uuid) -> Result<Option<BackupSchedule>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, institution_id, schedule_type, backup_type, interval_hours, 
         retention_days, is_active, next_run_at, last_run_at
         FROM backup_schedules WHERE institution_id = $1 AND is_active = true",
        institution_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| BackupSchedule {
        id: r.id,
        institution_id: r.institution_id,
        schedule_type: ScheduleType::Daily,
        backup_type: BackupType::Full,
        interval_hours: r.interval_hours,
        retention_days: r.retention_days,
        is_active: r.is_active,
        next_run_at: r.next_run_at,
        last_run_at: r.last_run_at,
    }))
}