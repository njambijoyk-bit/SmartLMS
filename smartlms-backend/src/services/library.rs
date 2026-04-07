// Library & Content Repository Service - Digital library, documents, media
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Library item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub item_type: LibraryItemType,
    pub category: String,
    pub tags: Vec<String>,
    pub file_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub view_count: i64,
    pub download_count: i64,
    pub is_published: bool,
    pub created_by: uuid::Uuid,
    pub created_at: DateTime<Utc>,
}

/// Library item types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LibraryItemType {
    Document,
    Video,
    Audio,
    Image,
    EBook,
    Article,
    Archive,
}

/// Library collection/folder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryCollection {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<uuid::Uuid>,
    pub item_count: i64,
    pub created_by: uuid::Uuid,
    pub created_at: DateTime<Utc>,
}

/// User's saved/favorited items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryBookmark {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub item_id: uuid::Uuid,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Item access/permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAccess {
    pub item_id: uuid::Uuid,
    pub access_type: AccessType,
    pub allowed_roles: Vec<String>,
    pub allowed_users: Vec<uuid::Uuid>,
}

/// Access types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessType {
    Public,
    Restricted,
    Private,
}

// Service functions
pub mod service {
    use super::*;

    /// Upload/create library item
    pub async fn create_item(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        creator_id: uuid::Uuid,
        req: &CreateItemRequest,
    ) -> Result<LibraryItem, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO library_items (id, institution_id, title, description, item_type,
             category, tags, file_url, thumbnail_url, metadata, view_count, download_count,
             is_published, created_by, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, 0, $11, $12, $13)",
            id,
            institution_id,
            req.title,
            req.description,
            format!("{:?}", req.item_type).to_lowercase(),
            req.category,
            serde_json::to_string(&req.tags).unwrap(),
            req.file_url,
            req.thumbnail_url,
            serde_json::to_string(&req.metadata).unwrap(),
            req.is_published,
            creator_id,
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(LibraryItem {
            id,
            institution_id,
            title: req.title.clone(),
            description: req.description.clone(),
            item_type: req.item_type,
            category: req.category.clone(),
            tags: req.tags.clone(),
            file_url: req.file_url.clone(),
            thumbnail_url: req.thumbnail_url.clone(),
            metadata: req.metadata.clone(),
            view_count: 0,
            download_count: 0,
            is_published: req.is_published,
            created_by: creator_id,
            created_at: Utc::now(),
        })
    }

    /// Search library items
    pub async fn search_items(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        query: &str,
        item_type: Option<LibraryItemType>,
        category: Option<&str>,
        tags: Option<Vec<String>>,
        limit: i64,
    ) -> Result<Vec<LibraryItem>, String> {
        let search_pattern = format!("%{}%", query);

        let rows = sqlx::query!(
            "SELECT id, institution_id, title, description, item_type, category, tags,
             file_url, thumbnail_url, metadata, view_count, download_count, 
             is_published, created_by, created_at
             FROM library_items 
             WHERE institution_id = $1 AND is_published = true 
             AND (title ILIKE $2 OR description ILIKE $2)
             ORDER BY created_at DESC LIMIT $3",
            institution_id,
            search_pattern,
            limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| LibraryItem {
                id: r.id,
                institution_id: r.institution_id,
                title: r.title,
                description: r.description,
                item_type: LibraryItemType::Document,
                category: r.category,
                tags: serde_json::from_str(&r.tags).unwrap_or_default(),
                file_url: r.file_url,
                thumbnail_url: r.thumbnail_url,
                metadata: serde_json::from_str(&r.metadata).unwrap_or_default(),
                view_count: r.view_count,
                download_count: r.download_count,
                is_published: r.is_published,
                created_by: r.created_by,
                created_at: r.created_at,
            })
            .collect())
    }

    /// Get item by ID
    pub async fn get_item(
        pool: &PgPool,
        item_id: uuid::Uuid,
    ) -> Result<Option<LibraryItem>, String> {
        let row = sqlx::query!(
            "SELECT id, institution_id, title, description, item_type, category, tags,
             file_url, thumbnail_url, metadata, view_count, download_count, 
             is_published, created_by, created_at
             FROM library_items WHERE id = $1",
            item_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| LibraryItem {
            id: r.id,
            institution_id: r.institution_id,
            title: r.title,
            description: r.description,
            item_type: LibraryItemType::Document,
            category: r.category,
            tags: serde_json::from_str(&r.tags).unwrap_or_default(),
            file_url: r.file_url,
            thumbnail_url: r.thumbnail_url,
            metadata: serde_json::from_str(&r.metadata).unwrap_or_default(),
            view_count: r.view_count,
            download_count: r.download_count,
            is_published: r.is_published,
            created_by: r.created_by,
            created_at: r.created_at,
        }))
    }

    /// Record view
    pub async fn record_view(pool: &PgPool, item_id: uuid::Uuid) -> Result<(), String> {
        sqlx::query!(
            "UPDATE library_items SET view_count = view_count + 1 WHERE id = $1",
            item_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Record download
    pub async fn record_download(pool: &PgPool, item_id: uuid::Uuid) -> Result<(), String> {
        sqlx::query!(
            "UPDATE library_items SET download_count = download_count + 1 WHERE id = $1",
            item_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Create collection
    pub async fn create_collection(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        creator_id: uuid::Uuid,
        name: &str,
        description: Option<&str>,
        parent_id: Option<uuid::Uuid>,
    ) -> Result<LibraryCollection, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO library_collections (id, institution_id, name, description, 
             parent_id, item_count, created_by, created_at)
             VALUES ($1, $2, $3, $4, $5, 0, $6, $7)",
            id,
            institution_id,
            name,
            description,
            parent_id,
            creator_id,
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(LibraryCollection {
            id,
            institution_id,
            name: name.to_string(),
            description: description.map(String::from),
            parent_id,
            item_count: 0,
            created_by: creator_id,
            created_at: Utc::now(),
        })
    }

    /// Get user's bookmarks
    pub async fn get_bookmarks(
        pool: &PgPool,
        user_id: uuid::Uuid,
    ) -> Result<Vec<LibraryBookmark>, String> {
        let rows = sqlx::query!(
            "SELECT id, user_id, item_id, note, created_at FROM library_bookmarks WHERE user_id = $1",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| LibraryBookmark {
                id: r.id,
                user_id: r.user_id,
                item_id: r.item_id,
                note: r.note,
                created_at: r.created_at,
            })
            .collect())
    }

    /// Add bookmark
    pub async fn add_bookmark(
        pool: &PgPool,
        user_id: uuid::Uuid,
        item_id: uuid::Uuid,
        note: Option<&str>,
    ) -> Result<LibraryBookmark, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO library_bookmarks (id, user_id, item_id, note, created_at)
             VALUES ($1, $2, $3, $4, $5)",
            id,
            user_id,
            item_id,
            note,
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(LibraryBookmark {
            id,
            user_id,
            item_id,
            note: note.map(String::from),
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub title: String,
    pub description: Option<String>,
    pub item_type: LibraryItemType,
    pub category: String,
    pub tags: Vec<String>,
    pub file_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub is_published: bool,
}
