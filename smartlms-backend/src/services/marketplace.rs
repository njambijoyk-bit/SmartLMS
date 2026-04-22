// Phase 17 Enhancement: Developer Marketplace
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppCategory {
    Analytics,
    Assessment,
    Communication,
    ContentManagement,
    Gamification,
    Integration,
    Productivity,
    Security,
    StudentEngagement,
    Other,
}

impl AppCategory {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "analytics" => AppCategory::Analytics,
            "assessment" => AppCategory::Assessment,
            "communication" => AppCategory::Communication,
            "content management" | "contentmanagement" => AppCategory::ContentManagement,
            "gamification" => AppCategory::Gamification,
            "integration" => AppCategory::Integration,
            "productivity" => AppCategory::Productivity,
            "security" => AppCategory::Security,
            "student engagement" | "studentengagement" => AppCategory::StudentEngagement,
            _ => AppCategory::Other,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            AppCategory::Analytics => "Analytics",
            AppCategory::Assessment => "Assessment",
            AppCategory::Communication => "Communication",
            AppCategory::ContentManagement => "Content Management",
            AppCategory::Gamification => "Gamification",
            AppCategory::Integration => "Integration",
            AppCategory::Productivity => "Productivity",
            AppCategory::Security => "Security",
            AppCategory::StudentEngagement => "Student Engagement",
            AppCategory::Other => "Other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppStatus {
    Draft,
    PendingReview,
    Approved,
    Rejected,
    Suspended,
    Archived,
}

impl AppStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppStatus::Draft => "draft",
            AppStatus::PendingReview => "pending_review",
            AppStatus::Approved => "approved",
            AppStatus::Rejected => "rejected",
            AppStatus::Suspended => "suspended",
            AppStatus::Archived => "archived",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppListing {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub developer_id: Uuid,
    pub developer_name: String,
    pub price: Option<f64>,
    pub is_free: bool,
    pub documentation_url: Option<String>,
    pub support_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub terms_url: Option<String>,
    pub icon_url: Option<String>,
    pub screenshot_urls: Vec<String>,
    pub rating: f64,
    pub review_count: i32,
    pub install_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInstallation {
    pub id: Uuid,
    pub app_id: Uuid,
    pub institution_id: Option<Uuid>,
    pub user_id: Uuid,
    pub api_key: String,
    pub webhook_secret: String,
    pub configuration: Option<serde_json::Value>,
    pub status: String,
    pub installed_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppReview {
    pub id: Uuid,
    pub app_id: Uuid,
    pub user_id: Uuid,
    pub rating: u8,
    pub title: String,
    pub comment: String,
    pub helpful_count: i32,
    pub created_at: DateTime<Utc>,
}

pub struct MarketplaceService;

impl MarketplaceService {
    /// List marketplace apps with optional filtering
    pub fn list_apps(
        category: Option<&str>,
        search: Option<&str>,
        sort: &str,
    ) -> Result<Vec<AppListing>, String> {
        // TODO: Query database with filters
        Ok(vec![])
    }
    
    /// Get app by ID
    pub fn get_app(id: &Uuid) -> Option<AppListing> {
        // TODO: Fetch from database
        None
    }
    
    /// Install an app
    pub fn install_app(
        app_id: &Uuid,
        institution_id: Option<Uuid>,
        configuration: Option<serde_json::Value>,
    ) -> Result<(Uuid, String), String> {
        let installation_id = Uuid::new_v4();
        let api_key = format!("sk_marketplace_{}", Uuid::new_v4());
        
        // TODO: Save installation to database
        
        Ok((installation_id, api_key))
    }
    
    /// Uninstall an app
    pub fn uninstall_app(installation_id: &Uuid) -> Result<(), String> {
        // TODO: Remove installation from database
        Ok(())
    }
    
    /// Submit a review for an app
    pub fn submit_review(
        app_id: &Uuid,
        user_id: &Uuid,
        rating: u8,
        title: String,
        comment: String,
    ) -> Result<AppReview, String> {
        if rating < 1 || rating > 5 {
            return Err("Rating must be between 1 and 5".to_string());
        }
        
        let review = AppReview {
            id: Uuid::new_v4(),
            app_id: *app_id,
            user_id: *user_id,
            rating,
            title,
            comment,
            helpful_count: 0,
            created_at: Utc::now(),
        };
        
        // TODO: Save review to database
        
        Ok(review)
    }
    
    /// Get reviews for an app
    pub fn get_app_reviews(app_id: &Uuid) -> Result<Vec<AppReview>, String> {
        // TODO: Fetch from database
        Ok(vec![])
    }
    
    /// Create a new app listing
    pub fn create_listing(
        developer_id: Uuid,
        developer_name: String,
        name: String,
        description: String,
        category: String,
        version: String,
        is_free: bool,
        price: Option<f64>,
    ) -> Result<AppListing, String> {
        let now = Utc::now();
        let listing = AppListing {
            id: Uuid::new_v4(),
            name,
            description,
            category,
            version,
            developer_id,
            developer_name,
            price,
            is_free,
            documentation_url: None,
            support_url: None,
            privacy_policy_url: None,
            terms_url: None,
            icon_url: None,
            screenshot_urls: vec![],
            rating: 0.0,
            review_count: 0,
            install_count: 0,
            status: AppStatus::Draft.as_str().to_string(),
            created_at: now,
            updated_at: now,
        };
        
        // TODO: Save to database
        
        Ok(listing)
    }
    
    /// Update an existing app listing
    pub fn update_listing(
        id: &Uuid,
        updates: serde_json::Value,
    ) -> Result<AppListing, String> {
        // TODO: Update in database
        Err("Not implemented".to_string())
    }
    
    /// Delete an app listing
    pub fn delete_listing(id: &Uuid) -> Result<(), String> {
        // TODO: Delete from database
        Ok(())
    }
    
    /// Get installations for a user or institution
    pub fn get_installations(
        user_id: Option<&Uuid>,
        institution_id: Option<&Uuid>,
    ) -> Result<Vec<AppInstallation>, String> {
        // TODO: Fetch from database
        Ok(vec![])
    }
}
