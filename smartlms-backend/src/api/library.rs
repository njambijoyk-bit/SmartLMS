//! Library & Content Repository API Endpoints
//! 
//! Provides REST endpoints for digital library management,
//! resource discovery, borrowing, and citations.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::user::UserClaims;
use crate::services::library::{LibraryService, LibraryError};
use crate::api::ApiResponse;

/// Resource search parameters
#[derive(Debug, Deserialize)]
pub struct ResourceSearchParams {
    pub query: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub resource_type: Option<String>,
    pub format: Option<String>,
    pub language: Option<String>,
    pub publication_year_from: Option<i32>,
    pub publication_year_to: Option<i32>,
    pub course_id: Option<i64>,
    pub collection_id: Option<i64>,
    pub is_open_access: Option<bool>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Citation request
#[derive(Debug, Deserialize)]
pub struct CitationRequest {
    pub resource_id: i64,
    pub style: String,
}

/// Bulk upload request
#[derive(Debug, Deserialize)]
pub struct BulkUploadRequest {
    pub collection_id: Option<i64>,
    pub course_id: Option<i64>,
    pub resources: Vec<BulkResourceEntry>,
}

#[derive(Debug, Deserialize)]
pub struct BulkResourceEntry {
    pub title: String,
    pub author: String,
    pub resource_type: String,
    pub format: String,
    pub url: Option<String>,
    pub file_path: Option<String>,
    pub description: Option<String>,
    pub subject: Option<Vec<String>>,
    pub language: Option<String>,
    pub publication_year: Option<i32>,
    pub publisher: Option<String>,
    pub isbn: Option<String>,
    pub doi: Option<String>,
    pub license: Option<String>,
    pub is_open_access: Option<bool>,
}

/// Create/update resource request
#[derive(Debug, Deserialize)]
pub struct CreateResourceRequest {
    pub title: String,
    pub author: String,
    pub resource_type: String,
    pub format: String,
    pub url: Option<String>,
    pub file_path: Option<String>,
    pub description: Option<String>,
    pub subject: Option<Vec<String>>,
    pub language: Option<String>,
    pub publication_year: Option<i32>,
    pub publisher: Option<String>,
    pub isbn: Option<String>,
    pub issn: Option<String>,
    pub doi: Option<String>,
    pub pmid: Option<String>,
    pub arxiv_id: Option<String>,
    pub license: Option<String>,
    pub is_open_access: bool,
    pub collection_ids: Option<Vec<i64>>,
    pub course_ids: Option<Vec<i64>>,
    pub metadata: Option<serde_json::Value>,
}

/// Update resource request
#[derive(Debug, Deserialize)]
pub struct UpdateResourceRequest {
    pub title: Option<String>,
    pub author: Option<String>,
    pub resource_type: Option<String>,
    pub format: Option<String>,
    pub url: Option<String>,
    pub file_path: Option<String>,
    pub description: Option<String>,
    pub subject: Option<Vec<String>>,
    pub language: Option<String>,
    pub publication_year: Option<i32>,
    pub publisher: Option<String>,
    pub isbn: Option<String>,
    pub issn: Option<String>,
    pub doi: Option<String>,
    pub pmid: Option<String>,
    pub arxiv_id: Option<String>,
    pub license: Option<String>,
    pub is_open_access: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

/// Create collection request
#[derive(Debug, Deserialize)]
pub struct CreateCollectionRequest {
    pub name: String,
    pub description: Option<String>,
    pub parent_collection_id: Option<i64>,
    pub collection_type: Option<String>,
    pub subject: Option<Vec<String>>,
    pub curator_id: Option<i64>,
    pub is_public: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

/// Update collection request
#[derive(Debug, Deserialize)]
pub struct UpdateCollectionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_collection_id: Option<i64>,
    pub collection_type: Option<String>,
    pub subject: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

/// Add resource to collection request
#[derive(Debug, Deserialize)]
pub struct AddToCollectionRequest {
    pub resource_id: i64,
    pub collection_id: i64,
}

/// Link resource to course request
#[derive(Debug, Deserialize)]
pub struct LinkResourceToCourseRequest {
    pub resource_id: i64,
    pub course_id: i64,
    pub linkage_type: String,
    pub module_id: Option<i64>,
    pub week: Option<i32>,
    pub is_required: Option<bool>,
}

/// Borrow physical item request
#[derive(Debug, Deserialize)]
pub struct BorrowItemRequest {
    pub resource_id: i64,
    pub user_id: i64,
    pub due_date: Option<String>,
}

/// Return item request
#[derive(Debug, Deserialize)]
pub struct ReturnItemRequest {
    pub loan_id: i64,
    pub condition_notes: Option<String>,
}

/// Renew loan request
#[derive(Debug, Deserialize)]
pub struct RenewLoanRequest {
    pub loan_id: i64,
    pub additional_days: Option<i32>,
}

/// OPDS feed parameters
#[derive(Debug, Deserialize)]
pub struct OpdsParams {
    pub collection_id: Option<i64>,
    pub search: Option<String>,
    pub page: Option<i32>,
}

// ============================================================================
// API Handlers
// ============================================================================

/// Search library resources
pub async fn search_resources(
    State(pool): State<PgPool>,
    Query(params): Query<ResourceSearchParams>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = LibraryService::new(pool);
    
    match service.search_resources(
        params.query,
        params.title,
        params.author,
        params.subject,
        params.resource_type,
        params.format,
        params.language,
        params.publication_year_from,
        params.publication_year_to,
        params.course_id,
        params.collection_id,
        params.is_open_access,
        params.page.unwrap_or(1),
        params.limit.unwrap_or(20),
        params.sort_by,
        params.sort_order,
    ).await {
        Ok(resources) => ApiResponse::success(resources),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get resource by ID
pub async fn get_resource(
    State(pool): State<PgPool>,
    Path(resource_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.get_resource_by_id(resource_id).await {
        Ok(resource) => ApiResponse::success(resource),
        Err(LibraryError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Create new resource
pub async fn create_resource(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<CreateResourceRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.create_resource(
        claims.user_id,
        req.title,
        req.author,
        req.resource_type,
        req.format,
        req.url,
        req.file_path,
        req.description,
        req.subject,
        req.language,
        req.publication_year,
        req.publisher,
        req.isbn,
        req.issn,
        req.doi,
        req.pmid,
        req.arxiv_id,
        req.license,
        req.is_open_access,
        req.collection_ids,
        req.course_ids,
        req.metadata,
    ).await {
        Ok(resource) => ApiResponse::created(resource),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Update resource
pub async fn update_resource(
    State(pool): State<PgPool>,
    Path(resource_id): Path<i64>,
    Json(req): Json<UpdateResourceRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.update_resource(
        resource_id,
        req.title,
        req.author,
        req.resource_type,
        req.format,
        req.url,
        req.file_path,
        req.description,
        req.subject,
        req.language,
        req.publication_year,
        req.publisher,
        req.isbn,
        req.issn,
        req.doi,
        req.pmid,
        req.arxiv_id,
        req.license,
        req.is_open_access,
        req.metadata,
    ).await {
        Ok(resource) => ApiResponse::success(resource),
        Err(LibraryError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Delete resource
pub async fn delete_resource(
    State(pool): State<PgPool>,
    Path(resource_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.delete_resource(resource_id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({"deleted": true})),
        Err(LibraryError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Generate citation
pub async fn generate_citation(
    State(pool): State<PgPool>,
    Json(req): Json<CitationRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.generate_citation(req.resource_id, req.style).await {
        Ok(citation) => ApiResponse::success(citation),
        Err(LibraryError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Bulk upload resources
pub async fn bulk_upload_resources(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<BulkUploadRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.bulk_upload_resources(
        claims.user_id,
        req.collection_id,
        req.course_id,
        req.resources,
    ).await {
        Ok(result) => ApiResponse::success(result),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get all collections
pub async fn get_collections(
    State(pool): State<PgPool>,
    Query(params): Query<serde_json::Value>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = LibraryService::new(pool);
    
    let parent_id = params.get("parent_id").and_then(|v| v.as_i64());
    let collection_type = params.get("type").and_then(|v| v.as_str()).map(String::from);
    
    match service.get_collections(parent_id, collection_type).await {
        Ok(collections) => ApiResponse::success(collections),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get collection by ID with hierarchy
pub async fn get_collection(
    State(pool): State<PgPool>,
    Path(collection_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.get_collection_by_id(collection_id).await {
        Ok(collection) => ApiResponse::success(collection),
        Err(LibraryError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Create collection
pub async fn create_collection(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<CreateCollectionRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.create_collection(
        claims.user_id,
        req.name,
        req.description,
        req.parent_collection_id,
        req.collection_type,
        req.subject,
        req.curator_id,
        req.is_public,
        req.metadata,
    ).await {
        Ok(collection) => ApiResponse::created(collection),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Update collection
pub async fn update_collection(
    State(pool): State<PgPool>,
    Path(collection_id): Path<i64>,
    Json(req): Json<UpdateCollectionRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.update_collection(
        collection_id,
        req.name,
        req.description,
        req.parent_collection_id,
        req.collection_type,
        req.subject,
        req.is_public,
        req.metadata,
    ).await {
        Ok(collection) => ApiResponse::success(collection),
        Err(LibraryError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Delete collection
pub async fn delete_collection(
    State(pool): State<PgPool>,
    Path(collection_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.delete_collection(collection_id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({"deleted": true})),
        Err(LibraryError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Add resource to collection
pub async fn add_resource_to_collection(
    State(pool): State<PgPool>,
    Json(req): Json<AddToCollectionRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.add_resource_to_collection(req.resource_id, req.collection_id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({"added": true})),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Remove resource from collection
pub async fn remove_resource_from_collection(
    State(pool): State<PgPool>,
    Path((collection_id, resource_id)): Path<(i64, i64)>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.remove_resource_from_collection(resource_id, collection_id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({"removed": true})),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get resources in collection
pub async fn get_collection_resources(
    State(pool): State<PgPool>,
    Path(collection_id): Path<i64>,
    Query(params): Query<serde_json::Value>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = LibraryService::new(pool);
    
    let page = params.get("page").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
    let limit = params.get("limit").and_then(|v| v.as_i64()).unwrap_or(20) as i32;
    
    match service.get_collection_resources(collection_id, page, limit).await {
        Ok(resources) => ApiResponse::success(resources),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Link resource to course
pub async fn link_resource_to_course(
    State(pool): State<PgPool>,
    Json(req): Json<LinkResourceToCourseRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.link_resource_to_course(
        req.resource_id,
        req.course_id,
        req.linkage_type,
        req.module_id,
        req.week,
        req.is_required,
    ).await {
        Ok(link) => ApiResponse::created(link),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Unlink resource from course
pub async fn unlink_resource_from_course(
    State(pool): State<PgPool>,
    Path((course_id, resource_id)): Path<(i64, i64)>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.unlink_resource_from_course(resource_id, course_id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({"unlinked": true})),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get course resources
pub async fn get_course_resources(
    State(pool): State<PgPool>,
    Path(course_id): Path<i64>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = LibraryService::new(pool);
    
    match service.get_course_resources(course_id).await {
        Ok(resources) => ApiResponse::success(resources),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Borrow physical item
pub async fn borrow_item(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<BorrowItemRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.borrow_physical_item(
        req.resource_id,
        req.user_id,
        claims.user_id,
        req.due_date,
    ).await {
        Ok(loan) => ApiResponse::created(loan),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Return physical item
pub async fn return_item(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<ReturnItemRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.return_physical_item(req.loan_id, claims.user_id, req.condition_notes).await {
        Ok(loan) => ApiResponse::success(loan),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Renew loan
pub async fn renew_loan(
    State(pool): State<PgPool>,
    Json(req): Json<RenewLoanRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.renew_loan(req.loan_id, req.additional_days).await {
        Ok(loan) => ApiResponse::success(loan),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get user loans
pub async fn get_user_loans(
    State(pool): State<PgPool>,
    Path(user_id): Path<i64>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = LibraryService::new(pool);
    
    match service.get_user_loans(user_id).await {
        Ok(loans) => ApiResponse::success(loans),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get overdue loans
pub async fn get_overdue_loans(
    State(pool): State<PgPool>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = LibraryService::new(pool);
    
    match service.get_overdue_loans().await {
        Ok(loans) => ApiResponse::success(loans),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Generate OPDS feed
pub async fn generate_opds_feed(
    State(pool): State<PgPool>,
    Query(params): Query<OpdsParams>,
) -> ApiResponse<String> {
    let service = LibraryService::new(pool);
    
    match service.generate_opds_feed(params.collection_id, params.search, params.page).await {
        Ok(feed) => ApiResponse::xml(feed),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get library statistics
pub async fn get_library_stats(
    State(pool): State<PgPool>,
) -> ApiResponse<serde_json::Value> {
    let service = LibraryService::new(pool);
    
    match service.get_library_statistics().await {
        Ok(stats) => ApiResponse::success(stats),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get popular resources
pub async fn get_popular_resources(
    State(pool): State<PgPool>,
    Query(params): Query<serde_json::Value>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = LibraryService::new(pool);
    
    let limit = params.get("limit").and_then(|v| v.as_i64()).unwrap_or(10) as i32;
    let days = params.get("days").and_then(|v| v.as_i64()).unwrap_or(30) as i32;
    
    match service.get_popular_resources(limit, days).await {
        Ok(resources) => ApiResponse::success(resources),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get recent additions
pub async fn get_recent_additions(
    State(pool): State<PgPool>,
    Query(params): Query<serde_json::Value>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = LibraryService::new(pool);
    
    let limit = params.get("limit").and_then(|v| v.as_i64()).unwrap_or(20) as i32;
    let days = params.get("days").and_then(|v| v.as_i64()).unwrap_or(7) as i32;
    
    match service.get_recent_additions(limit, days).await {
        Ok(resources) => ApiResponse::success(resources),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

// ============================================================================
// Router Configuration
// ============================================================================

use axum::Router;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use crate::middleware::auth::AuthMiddleware;

/// Create library router
pub fn library_router() -> Router {
    Router::new()
        // Resource endpoints
        .route("/resources", axum::routing::get(search_resources))
        .route("/resources", axum::routing::post(create_resource))
        .route("/resources/:resource_id", axum::routing::get(get_resource))
        .route("/resources/:resource_id", axum::routing::put(update_resource))
        .route("/resources/:resource_id", axum::routing::delete(delete_resource))
        .route("/resources/citation", axum::routing::post(generate_citation))
        .route("/resources/bulk-upload", axum::routing::post(bulk_upload_resources))
        // Collection endpoints
        .route("/collections", axum::routing::get(get_collections))
        .route("/collections/:collection_id", axum::routing::get(get_collection))
        .route("/collections", axum::routing::post(create_collection))
        .route("/collections/:collection_id", axum::routing::put(update_collection))
        .route("/collections/:collection_id", axum::routing::delete(delete_collection))
        .route("/collections/add-resource", axum::routing::post(add_resource_to_collection))
        .route("/collections/:collection_id/resources/:resource_id", axum::routing::delete(remove_resource_from_collection))
        .route("/collections/:collection_id/resources", axum::routing::get(get_collection_resources))
        // Course linkage endpoints
        .route("/link-course", axum::routing::post(link_resource_to_course))
        .route("/courses/:course_id/resources/:resource_id", axum::routing::delete(unlink_resource_from_course))
        .route("/courses/:course_id/resources", axum::routing::get(get_course_resources))
        // Physical borrowing endpoints
        .route("/borrow", axum::routing::post(borrow_item))
        .route("/return", axum::routing::post(return_item))
        .route("/renew", axum::routing::post(renew_loan))
        .route("/loans/:user_id", axum::routing::get(get_user_loans))
        .route("/loans/overdue", axum::routing::get(get_overdue_loans))
        // OPDS feed
        .route("/opds", axum::routing::get(generate_opds_feed))
        // Statistics
        .route("/stats", axum::routing::get(get_library_stats))
        .route("/popular", axum::routing::get(get_popular_resources))
        .route("/recent", axum::routing::get(get_recent_additions))
        .layer(AsyncRequireAuthorizationLayer::new(AuthMiddleware::new()))
}
