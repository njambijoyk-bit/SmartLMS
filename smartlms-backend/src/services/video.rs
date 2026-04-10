// Video Service - Upload, storage, and HLS transcoding
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Video asset status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoStatus {
    Uploaded,     // Initial upload complete
    Processing,   // Being transcoded
    Ready,        // HLS available
    Failed,       // Transcoding failed
    Deleted,      // Soft deleted
}

/// Video asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub lesson_id: Option<uuid::Uuid>,
    pub title: String,
    pub description: Option<String>,
    
    // Original file
    pub original_filename: String,
    pub original_size_bytes: i64,
    pub original_url: Option<String>,
    
    // HLS output
    pub hls_playlist_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    
    // Quality variants (bitrates)
    pub variants: Vec<VideoVariant>,
    
    pub status: VideoStatus,
    pub created_by: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

/// Video quality variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoVariant {
    pub resolution: String,    // "1920x1080", "1280x720", etc.
    pub bitrate_kbps: i32,
    pub url: String,
    pub file_size_bytes: i64,
}

/// Upload configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadConfig {
    pub upload_url: String,
    pub upload_method: String,
    pub max_file_size_mb: i64,
    pub allowed_formats: Vec<String>,
    pub expires_in_seconds: i64,
    pub video_id: String,
}

/// Upload progress event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodeJob {
    pub id: uuid::Uuid,
    pub video_id: uuid::Uuid,
    pub status: VideoStatus,
    pub progress_percent: i32,
    pub current_step: String,
    pub error_message: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

// Service functions
pub mod service {
    use super::*;
    
    /// Create upload configuration for direct upload
    pub async fn create_upload(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        creator_id: uuid::Uuid,
        filename: &str,
        content_type: &str,
    ) -> Result<UploadConfig, String> {
        // Validate file type
        let allowed = vec!["video/mp4", "video/webm", "video/quicktime", "video/x-msvideo"];
        if !allowed.contains(&content_type) {
            return Err("Invalid video format. Allowed: MP4, WebM, MOV, AVI".to_string());
        }
        
        let video_id = Uuid::new_v4();
        
        // Generate pre-signed URL for direct upload to storage (S3/GCS)
        // In production, this would call cloud storage API
        let upload_url = format!("/api/v1/videos/{}/upload", video_id);
        
        // Create video record
        sqlx::query!(
            "INSERT INTO videos (id, institution_id, original_filename, original_size_bytes, 
             status, created_by, created_at)
             VALUES ($1, $2, $3, 0, 'uploaded', $4, $5)",
            video_id, institution_id, filename, creator_id, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(UploadConfig {
            upload_url,
            upload_method: "PUT".to_string(),
            max_file_size_mb: 2048,  // 2GB max
            allowed_formats: vec!["mp4".to_string(), "webm".to_string(), "mov".to_string()],
            expires_in_seconds: 3600,
            video_id: video_id.to_string(),
        })
    }
    
    /// Confirm upload complete, start transcoding
    pub async fn confirm_upload(
        pool: &PgPool,
        video_id: uuid::Uuid,
        file_size_bytes: i64,
    ) -> Result<TranscodeJob, String> {
        // Update file size
        sqlx::query!(
            "UPDATE videos SET original_size_bytes = $1, status = 'processing' WHERE id = $2",
            file_size_bytes, video_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // Create transcode job
        let job_id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query!(
            "INSERT INTO transcode_jobs (id, video_id, status, progress_percent, current_step, started_at)
             VALUES ($1, $2, 'processing', 0, 'Starting transcoding', $3)",
            job_id, video_id, now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // In production: trigger external transcoder (AWS MediaConvert, FFmpeg, etc.)
        // For now, simulate async processing
        
        Ok(TranscodeJob {
            id: job_id,
            video_id,
            status: VideoStatus::Processing,
            progress_percent: 0,
            current_step: "Starting transcoding".to_string(),
            error_message: None,
            started_at: now,
            completed_at: None,
        })
    }
    
    /// Update transcode progress (called by worker)
    pub async fn update_transcode_progress(
        pool: &PgPool,
        job_id: uuid::Uuid,
        progress: i32,
        step: &str,
    ) -> Result<(), String> {
        sqlx::query!(
            "UPDATE transcode_jobs SET progress_percent = $1, current_step = $2 WHERE id = $3",
            progress, step, job_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    /// Complete transcoding (called by worker when done)
    pub async fn complete_transcoding(
        pool: &PgPool,
        video_id: uuid::Uuid,
        hls_url: &str,
        thumbnail_url: &str,
        duration_seconds: i32,
        variants: Vec<VideoVariant>,
    ) -> Result<Video, String> {
        let now = Utc::now();
        
        sqlx::query!(
            "UPDATE videos SET 
                status = 'ready',
                hls_playlist_url = $1,
                thumbnail_url = $2,
                duration_seconds = $3,
                processed_at = $4
             WHERE id = $5",
            hls_url, thumbnail_url, duration_seconds, now, video_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // Store variants (in production, insert into video_variants table)
        
        get_video(pool, video_id).await
    }
    
    /// Get video by ID
    pub async fn get_video(
        pool: &PgPool,
        video_id: uuid::Uuid,
    ) -> Result<Option<Video>, String> {
        let row = sqlx::query!(
            "SELECT id, institution_id, lesson_id, title, description, original_filename, 
             original_size_bytes, original_url, hls_playlist_url, thumbnail_url, duration_seconds,
             status, created_by, created_at, processed_at
             FROM videos WHERE id = $1",
            video_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(row.map(|r| Video {
            id: r.id,
            institution_id: r.institution_id,
            lesson_id: r.lesson_id,
            title: r.title,
            description: r.description,
            original_filename: r.original_filename,
            original_size_bytes: r.original_size_bytes,
            original_url: r.original_url,
            hls_playlist_url: r.hls_playlist_url,
            thumbnail_url: r.thumbnail_url,
            duration_seconds: r.duration_seconds,
            variants: vec![],
            status: VideoStatus::Ready,
            created_by: r.created_by,
            created_at: r.created_at,
            processed_at: r.processed_at,
        }))
    }
    
    /// Delete video (soft delete)
    pub async fn delete_video(
        pool: &PgPool,
        video_id: uuid::Uuid,
    ) -> Result<(), String> {
        sqlx::query!("UPDATE videos SET status = 'deleted' WHERE id = $1", video_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        
        // In production: also mark for deletion from storage
        Ok(())
    }
    
    /// Get signed URL for video playback
    pub async fn get_playback_url(
        pool: &PgPool,
        video_id: uuid::Uuid,
    ) -> Result<String, String> {
        let video = get_video(pool, video_id)
            .await?
            .ok_or("Video not found")?;
        
        if video.status != VideoStatus::Ready {
            return Err("Video not ready for playback".to_string());
        }
        
        // Generate signed URL (expires in 1 hour)
        // In production: use cloud storage signed URL
        Ok(video.hls_playlist_url.unwrap_or_default())
    }
    
    /// List videos for institution
    pub async fn list_videos(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Video>, String> {
        let rows = sqlx::query!(
            "SELECT id, institution_id, lesson_id, title, description, original_filename, 
             original_size_bytes, original_url, hls_playlist_url, thumbnail_url, duration_seconds,
             status, created_by, created_at, processed_at
             FROM videos WHERE institution_id = $1 AND status != 'deleted'
             ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            institution_id, limit, offset
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| Video {
            id: r.id,
            institution_id: r.institution_id,
            lesson_id: r.lesson_id,
            title: r.title,
            description: r.description,
            original_filename: r.original_filename,
            original_size_bytes: r.original_size_bytes,
            original_url: r.original_url,
            hls_playlist_url: r.hls_playlist_url,
            thumbnail_url: r.thumbnail_url,
            duration_seconds: r.duration_seconds,
            variants: vec![],
            status: VideoStatus::Ready,
            created_by: r.created_by,
            created_at: r.created_at,
            processed_at: r.processed_at,
        }).collect())
    }
}