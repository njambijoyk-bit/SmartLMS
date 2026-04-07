//! Course service

use uuid::Uuid;

/// Course filter options
#[derive(Debug, Default)]
pub struct CourseFilter {
    pub status: Option<String>,
    pub category: Option<String>,
    pub instructor_id: Option<Uuid>,
    pub search: Option<String>,
    pub page: usize,
    pub page_size: usize,
}

/// Paginated result
#[derive(Debug, serde::Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: usize, page: usize, page_size: usize) -> Self {
        let total_pages = (total + page_size - 1) / page_size;
        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}