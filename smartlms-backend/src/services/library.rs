// Library & Content Repository Service - Digital library, documents, media, citations, physical borrowing
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
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub publication_date: Option<DateTime<Utc>>,
    pub isbn: Option<String>,
    pub doi: Option<String>,
    pub language: Option<String>,
    pub page_count: Option<i32>,
    pub edition: Option<String>,
    pub subjects: Vec<String>,
    pub view_count: i64,
    pub download_count: i64,
    pub citation_count: i64,
    pub is_published: bool,
    pub allow_download: bool,
    pub access_level: AccessType,
    pub created_by: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    Thesis,
    Dataset,
    Software,
}

/// Library collection/folder with hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryCollection {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<uuid::Uuid>,
    pub path: String, // Full path like "/Engineering/Computer Science/AI"
    pub item_count: i64,
    pub subcollection_count: i64,
    pub created_by: uuid::Uuid,
    pub created_at: DateTime<Utc>,
}

/// User's saved/favorited items with notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryBookmark {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub item_id: uuid::Uuid,
    pub collection_id: Option<uuid::Uuid>,
    pub note: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Physical book/item borrowing record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowRecord {
    pub id: uuid::Uuid,
    pub item_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub borrowed_at: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
    pub status: BorrowStatus,
    pub renewals_count: i32,
    pub fines_amount: f64,
    pub notes: Option<String>,
}

/// Borrow status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorrowStatus {
    Borrowed,
    Returned,
    Overdue,
    Lost,
    Reserved,
}

/// Citation format types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitationStyle {
    APA,
    MLA,
    Chicago,
    IEEE,
    Harvard,
    Vancouver,
    BibTeX,
}

/// Generated citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub item_id: uuid::Uuid,
    pub style: CitationStyle,
    pub formatted: String,
    pub bibtex: Option<String>,
}

/// Bulk upload job for batch processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUploadJob {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub uploaded_by: uuid::Uuid,
    pub total_items: i32,
    pub processed_items: i32,
    pub failed_items: i32,
    pub status: UploadStatus,
    pub errors: Vec<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Upload status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UploadStatus {
    Pending,
    Processing,
    Completed,
    PartiallyCompleted,
    Failed,
}

/// OPDS feed entry for library discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpdsEntry {
    pub id: String,
    pub title: String,
    pub author: Vec<String>,
    pub summary: Option<String>,
    pub category: Vec<String>,
    pub published: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub links: Vec<OpdsLink>,
}

/// OPDS link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpdsLink {
    pub href: String,
    pub rel: String,
    pub mime_type: String,
    pub title: Option<String>,
}

/// Course linkage for library items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseLinkage {
    pub item_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub linked_by: uuid::Uuid,
    pub linkage_type: LinkageType,
    pub is_required: bool,
    pub week_number: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Linkage type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkageType {
    RequiredReading,
    RecommendedReading,
    SupplementaryMaterial,
    Reference,
    Assignment,
}

/// Search filters for advanced queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarySearchFilters {
    pub query: Option<String>,
    pub item_types: Vec<LibraryItemType>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub authors: Vec<String>,
    pub publication_year_from: Option<i32>,
    pub publication_year_to: Option<i32>,
    pub language: Option<String>,
    pub is_available: Option<bool>,
    pub access_level: Option<AccessType>,
    pub sort_by: SortField,
    pub sort_order: SortOrder,
    pub limit: i64,
    pub offset: i64,
}

/// Sort field options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortField {
    Relevance,
    Title,
    Author,
    PublicationDate,
    ViewCount,
    DownloadCount,
    CreatedAt,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Item access/permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAccess {
    pub item_id: uuid::Uuid,
    pub access_type: AccessType,
    pub allowed_roles: Vec<String>,
    pub allowed_users: Vec<uuid::Uuid>,
    pub allowed_courses: Vec<uuid::Uuid>,
    pub embargo_date: Option<DateTime<Utc>>,
}

/// Access types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessType {
    Public,
    Restricted,
    Private,
    InstitutionOnly,
    CourseOnly,
}

// Service functions
pub mod service {
    use super::*;
    use sqlx::PgPool;

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
             category, tags, file_url, thumbnail_url, metadata, authors, publisher, 
             publication_date, isbn, doi, language, page_count, edition, subjects,
             view_count, download_count, citation_count, is_published, allow_download,
             access_level, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, 0, 0, 0, $20, $21, $22, $23, $24, $25)",
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
            serde_json::to_string(&req.authors).unwrap(),
            req.publisher,
            req.publication_date,
            req.isbn,
            req.doi,
            req.language,
            req.page_count,
            req.edition,
            serde_json::to_string(&req.subjects).unwrap(),
            req.is_published,
            req.allow_download,
            format!("{:?}", req.access_level).to_lowercase(),
            creator_id,
            Utc::now(),
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
            authors: req.authors.clone(),
            publisher: req.publisher.clone(),
            publication_date: req.publication_date,
            isbn: req.isbn.clone(),
            doi: req.doi.clone(),
            language: req.language.clone(),
            page_count: req.page_count,
            edition: req.edition.clone(),
            subjects: req.subjects.clone(),
            view_count: 0,
            download_count: 0,
            citation_count: 0,
            is_published: req.is_published,
            allow_download: req.allow_download,
            access_level: req.access_level,
            created_by: creator_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Advanced search with full-text and filters
    pub async fn search_items(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        filters: &LibrarySearchFilters,
    ) -> Result<Vec<LibraryItem>, String> {
        let search_pattern = filters.query.as_ref().map(|q| format!("%{}%", q));

        let rows = sqlx::query!(
            "SELECT id, institution_id, title, description, item_type, category, tags,
             file_url, thumbnail_url, metadata, authors, publisher, publication_date,
             isbn, doi, language, page_count, edition, subjects,
             view_count, download_count, citation_count, 
             is_published, allow_download, access_level, created_by, created_at, updated_at
             FROM library_items 
             WHERE institution_id = $1 AND is_published = true 
             AND ($2::text IS NULL OR title ILIKE $2 OR description ILIKE $2)
             ORDER BY created_at DESC LIMIT $3 OFFSET $4",
            institution_id,
            search_pattern,
            filters.limit,
            filters.offset
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
                authors: serde_json::from_str(&r.authors).unwrap_or_default(),
                publisher: r.publisher,
                publication_date: r.publication_date,
                isbn: r.isbn,
                doi: r.doi,
                language: r.language,
                page_count: r.page_count,
                edition: r.edition,
                subjects: serde_json::from_str(&r.subjects).unwrap_or_default(),
                view_count: r.view_count,
                download_count: r.download_count,
                citation_count: r.citation_count,
                is_published: r.is_published,
                allow_download: r.allow_download,
                access_level: AccessType::Public,
                created_by: r.created_by,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    /// Generate citation in specified format
    pub async fn generate_citation(
        pool: &PgPool,
        item_id: uuid::Uuid,
        style: CitationStyle,
    ) -> Result<Citation, String> {
        let item = get_item(pool, item_id)
            .await?
            .ok_or("Item not found".to_string())?;

        let formatted = match style {
            CitationStyle::APA => {
                let authors_str = format_authors_apa(&item.authors);
                let year = item.publication_date.map(|d| d.year()).unwrap_or(0);
                format!(
                    "{} ({}){}. {}{}",
                    authors_str,
                    year,
                    if let Some(publisher) = &item.publisher {
                        format!(" {}", publisher)
                    } else {
                        String::new()
                    },
                    item.title,
                    if let Some(doi) = &item.doi {
                        format!(" https://doi.org/{}", doi)
                    } else {
                        String::new()
                    }
                )
            }
            CitationStyle::MLA => {
                let authors_str = format_authors_mla(&item.authors);
                format!(
                    "{}. \"{}\"{}. {}",
                    authors_str,
                    item.title,
                    item.publisher.as_ref().map(|p| format!(" {}", p)).unwrap_or_default(),
                    item.publication_date.map(|d| d.format("%Y")).unwrap_or_else(|| "n.d.".into())
                )
            }
            CitationStyle::Chicago => {
                let authors_str = format_authors_chicago(&item.authors);
                format!(
                    "{}. \"{}\"{}. {}",
                    authors_str,
                    item.title,
                    item.publisher.as_ref().map(|p| format!(" {}", p)).unwrap_or_default(),
                    item.publication_date.map(|d| d.format("%Y")).unwrap_or_else(|| "n.d.".into())
                )
            }
            CitationStyle::IEEE => {
                format!(
                    "[{}] {}. \"{}\". {}",
                    item.id.simple(),
                    format_authors_ieee(&item.authors),
                    item.title,
                    item.publication_date.map(|d| d.format("%Y")).unwrap_or_else(|| "n.d.".into())
                )
            }
            CitationStyle::Harvard => {
                let authors_str = format_authors_harvard(&item.authors);
                let year = item.publication_date.map(|d| d.year()).unwrap_or(0);
                format!(
                    "{} ({}) {}. {}",
                    authors_str,
                    year,
                    item.title,
                    item.publisher.as_ref().map(|p| format!("{}", p)).unwrap_or_default()
                )
            }
            CitationStyle::Vancouver => {
                format!(
                    "{}. {}. {}; {}",
                    format_authors_vancouver(&item.authors),
                    item.title,
                    item.publisher.as_ref().map(|p| format!("{}", p)).unwrap_or_default(),
                    item.publication_date.map(|d| d.format("%Y")).unwrap_or_else(|| "n.d.".into())
                )
            }
            CitationStyle::BibTeX => {
                let entry_type = match item.item_type {
                    LibraryItemType::EBook => "book",
                    LibraryItemType::Article => "article",
                    _ => "misc",
                };
                format!(
                    "@{}{{{},\n  title = {{{}}},\n  author = {{{}}},\n  year = {{{}}}\n}}",
                    entry_type,
                    item.id.simple(),
                    item.title,
                    item.authors.join(" and "),
                    item.publication_date.map(|d| d.year()).unwrap_or(0)
                )
            }
        };

        let bibtex = if style != CitationStyle::BibTeX {
            Some(format!(
                "@misc {{ {},\n  title = {{{}}},\n  author = {{{}}},\n  year = {{{}}}\n }}",
                item.id.simple(),
                item.title,
                item.authors.join(" and "),
                item.publication_date.map(|d| d.year()).unwrap_or(0)
            ))
        } else {
            None
        };

        // Increment citation count
        increment_citation_count(pool, item_id).await.ok();

        Ok(Citation {
            item_id,
            style,
            formatted,
            bibtex,
        })
    }

    fn format_authors_apa(authors: &[String]) -> String {
        if authors.is_empty() {
            return "Unknown".to_string();
        }
        if authors.len() == 1 {
            return authors[0].clone();
        }
        if authors.len() <= 7 {
            authors.join(", ")
        } else {
            format!("{}, ..., {}", authors.first().unwrap(), authors.last().unwrap())
        }
    }

    fn format_authors_mla(authors: &[String]) -> String {
        if authors.is_empty() {
            return "Unknown".to_string();
        }
        if authors.len() == 1 {
            return authors[0].clone();
        }
        if authors.len() == 2 {
            format!("{} and {}", authors[0], authors[1])
        } else {
            format!("{} et al.", authors.first().unwrap())
        }
    }

    fn format_authors_chicago(authors: &[String]) -> String {
        if authors.is_empty() {
            return "Unknown".to_string();
        }
        if authors.len() == 1 {
            return authors[0].clone();
        }
        format!("{} and {}", authors.first().unwrap(), authors.last().unwrap())
    }

    fn format_authors_ieee(authors: &[String]) -> String {
        if authors.is_empty() {
            return "Unknown".to_string();
        }
        authors.join(", ")
    }

    fn format_authors_harvard(authors: &[String]) -> String {
        if authors.is_empty() {
            return "Unknown".to_string();
        }
        if authors.len() == 1 {
            return authors[0].clone();
        }
        format!("{} et al.", authors.first().unwrap())
    }

    fn format_authors_vancouver(authors: &[String]) -> String {
        if authors.is_empty() {
            return "Unknown".to_string();
        }
        authors.join(", ")
    }

    async fn increment_citation_count(pool: &PgPool, item_id: uuid::Uuid) -> Result<(), String> {
        sqlx::query!(
            "UPDATE library_items SET citation_count = citation_count + 1 WHERE id = $1",
            item_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Get item by ID
    pub async fn get_item(
        pool: &PgPool,
        item_id: uuid::Uuid,
    ) -> Result<Option<LibraryItem>, String> {
        let row = sqlx::query!(
            "SELECT id, institution_id, title, description, item_type, category, tags,
             file_url, thumbnail_url, metadata, authors, publisher, publication_date,
             isbn, doi, language, page_count, edition, subjects,
             view_count, download_count, citation_count, 
             is_published, allow_download, access_level, created_by, created_at, updated_at
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
            authors: serde_json::from_str(&r.authors).unwrap_or_default(),
            publisher: r.publisher,
            publication_date: r.publication_date,
            isbn: r.isbn,
            doi: r.doi,
            language: r.language,
            page_count: r.page_count,
            edition: r.edition,
            subjects: serde_json::from_str(&r.subjects).unwrap_or_default(),
            view_count: r.view_count,
            download_count: r.download_count,
            citation_count: r.citation_count,
            is_published: r.is_published,
            allow_download: r.allow_download,
            access_level: AccessType::Public,
            created_by: r.created_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
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

    /// Create collection with hierarchical path
    pub async fn create_collection(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        creator_id: uuid::Uuid,
        name: &str,
        description: Option<&str>,
        parent_id: Option<uuid::Uuid>,
    ) -> Result<LibraryCollection, String> {
        let id = Uuid::new_v4();

        // Build path
        let path = if let Some(pid) = parent_id {
            let parent = get_collection(pool, pid)
                .await?
                .ok_or("Parent collection not found".to_string())?;
            format!("{}/{}", parent.path, name)
        } else {
            format!("/{}", name)
        };

        sqlx::query!(
            "INSERT INTO library_collections (id, institution_id, name, description, 
             parent_id, path, item_count, subcollection_count, created_by, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, 0, 0, $7, $8)",
            id,
            institution_id,
            name,
            description,
            parent_id,
            path,
            creator_id,
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Update parent's subcollection count
        if let Some(pid) = parent_id {
            sqlx::query!(
                "UPDATE library_collections SET subcollection_count = subcollection_count + 1 WHERE id = $1",
                pid
            )
            .execute(pool)
            .await
            .ok();
        }

        Ok(LibraryCollection {
            id,
            institution_id,
            name: name.to_string(),
            description: description.map(String::from),
            parent_id,
            path,
            item_count: 0,
            subcollection_count: 0,
            created_by: creator_id,
            created_at: Utc::now(),
        })
    }

    /// Get collection by ID
    pub async fn get_collection(
        pool: &PgPool,
        collection_id: uuid::Uuid,
    ) -> Result<Option<LibraryCollection>, String> {
        let row = sqlx::query!(
            "SELECT id, institution_id, name, description, parent_id, path, 
             item_count, subcollection_count, created_by, created_at
             FROM library_collections WHERE id = $1",
            collection_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| LibraryCollection {
            id: r.id,
            institution_id: r.institution_id,
            name: r.name,
            description: r.description,
            parent_id: r.parent_id,
            path: r.path,
            item_count: r.item_count,
            subcollection_count: r.subcollection_count,
            created_by: r.created_by,
            created_at: r.created_at,
        }))
    }

    /// Get user's bookmarks
    pub async fn get_bookmarks(
        pool: &PgPool,
        user_id: uuid::Uuid,
    ) -> Result<Vec<LibraryBookmark>, String> {
        let rows = sqlx::query!(
            "SELECT id, user_id, item_id, collection_id, note, tags, created_at, updated_at FROM library_bookmarks WHERE user_id = $1",
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
                collection_id: r.collection_id,
                note: r.note,
                tags: serde_json::from_str(&r.tags).unwrap_or_default(),
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    /// Add bookmark
    pub async fn add_bookmark(
        pool: &PgPool,
        user_id: uuid::Uuid,
        item_id: uuid::Uuid,
        collection_id: Option<uuid::Uuid>,
        note: Option<&str>,
        tags: Vec<String>,
    ) -> Result<LibraryBookmark, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO library_bookmarks (id, user_id, item_id, collection_id, note, tags, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            id,
            user_id,
            item_id,
            collection_id,
            note,
            serde_json::to_string(&tags).unwrap(),
            Utc::now(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(LibraryBookmark {
            id,
            user_id,
            item_id,
            collection_id,
            note: note.map(String::from),
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Borrow physical item
    pub async fn borrow_item(
        pool: &PgPool,
        item_id: uuid::Uuid,
        user_id: uuid::Uuid,
        loan_period_days: i32,
    ) -> Result<BorrowRecord, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let due_date = now + chrono::Duration::days(loan_period_days as i64);

        sqlx::query!(
            "INSERT INTO borrow_records (id, item_id, user_id, borrowed_at, due_date, status, renewals_count, fines_amount)
             VALUES ($1, $2, $3, $4, $5, 'borrowed', 0, 0.0)",
            id,
            item_id,
            user_id,
            now,
            due_date
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(BorrowRecord {
            id,
            item_id,
            user_id,
            borrowed_at: now,
            due_date,
            returned_at: None,
            status: BorrowStatus::Borrowed,
            renewals_count: 0,
            fines_amount: 0.0,
            notes: None,
        })
    }

    /// Return borrowed item
    pub async fn return_item(
        pool: &PgPool,
        borrow_id: uuid::Uuid,
    ) -> Result<BorrowRecord, String> {
        let now = Utc::now();

        // Get current record
        let record = sqlx::query!(
            "SELECT id, item_id, user_id, borrowed_at, due_date, status, renewals_count, fines_amount, notes
             FROM borrow_records WHERE id = $1",
            borrow_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Calculate fines if overdue
        let fines = if now > record.due_date {
            let days_overdue = (now - record.due_date).num_days() as f64;
            days_overdue * 0.50 // $0.50 per day
        } else {
            0.0
        };

        sqlx::query!(
            "UPDATE borrow_records SET returned_at = $1, status = 'returned', fines_amount = $2 WHERE id = $3",
            now,
            fines,
            borrow_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(BorrowRecord {
            id: record.id,
            item_id: record.item_id,
            user_id: record.user_id,
            borrowed_at: record.borrowed_at,
            due_date: record.due_date,
            returned_at: Some(now),
            status: BorrowStatus::Returned,
            renewals_count: record.renewals_count,
            fines_amount: fines,
            notes: record.notes,
        })
    }

    /// Link item to course
    pub async fn link_to_course(
        pool: &PgPool,
        item_id: uuid::Uuid,
        course_id: uuid::Uuid,
        linked_by: uuid::Uuid,
        linkage_type: LinkageType,
        is_required: bool,
        week_number: Option<i32>,
        notes: Option<&str>,
    ) -> Result<CourseLinkage, String> {
        let exists = sqlx::query!(
            "SELECT id FROM course_linkages WHERE item_id = $1 AND course_id = $2",
            item_id,
            course_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        if exists.is_some() {
            return Err("Item already linked to this course".to_string());
        }

        sqlx::query!(
            "INSERT INTO course_linkages (item_id, course_id, linked_by, linkage_type, is_required, week_number, notes, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            item_id,
            course_id,
            linked_by,
            format!("{:?}", linkage_type).to_lowercase(),
            is_required,
            week_number,
            notes,
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(CourseLinkage {
            item_id,
            course_id,
            linked_by,
            linkage_type,
            is_required,
            week_number,
            notes: notes.map(String::from),
            created_at: Utc::now(),
        })
    }

    /// Generate OPDS feed entry
    pub fn generate_opds_entry(item: &LibraryItem) -> OpdsEntry {
        OpdsEntry {
            id: item.id.to_string(),
            title: item.title.clone(),
            author: item.authors.clone(),
            summary: item.description.clone(),
            category: item.subjects.clone(),
            published: item.created_at,
            updated: item.updated_at,
            links: vec![
                OpdsLink {
                    href: format!("/api/library/items/{}", item.id),
                    rel: "self",
                    mime_type: "application/json".to_string(),
                    title: Some("Item Details".to_string()),
                },
                OpdsLink {
                    href: item.file_url.clone().unwrap_or_default(),
                    rel: "http://opds-spec.org/acquisition",
                    mime_type: "application/pdf".to_string(),
                    title: Some("Download".to_string()),
                },
            ],
        }
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
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub publication_date: Option<DateTime<Utc>>,
    pub isbn: Option<String>,
    pub doi: Option<String>,
    pub language: Option<String>,
    pub page_count: Option<i32>,
    pub edition: Option<String>,
    pub subjects: Vec<String>,
    pub is_published: bool,
    pub allow_download: bool,
    pub access_level: AccessType,
}
