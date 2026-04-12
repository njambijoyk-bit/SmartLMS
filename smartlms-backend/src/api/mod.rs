// API module - HTTP route handlers
pub mod abac;
pub mod ai_assistant;
pub mod analytics;
pub mod api_analytics;
pub mod assessments;
pub mod auth;
pub mod automation;
pub mod blockchain;
pub mod code_sandbox;
pub mod communication;
pub mod compliance;
pub mod courses;
pub mod course_groups;
pub mod enrollments;
pub mod employer;
pub mod gamification;
pub mod institutions;
pub mod iot;
pub mod iot_advanced;
pub mod library;
pub mod live;
pub mod marketplace;
pub mod mobile;
pub mod oauth;
pub mod sdk;
pub mod upgrade;
pub mod users;
pub mod developer;
pub mod vpat;
pub mod websocket;
pub mod parents_alumni;

/// Combine all routers into main API
pub fn create_api_router() -> axum::Router {
    axum::Router::new()
        .nest("/auth", auth::auth_router())
        .nest("/institutions", institutions::institutions_router())
        .nest("/courses", courses::courses_router())
        .nest("/course-groups", course_groups::course_groups_router())
        .nest("/assessments", assessments::assessments_router())
        .nest("/communication", communication::communication_router())
        .nest("/live", live::live_router())
        .nest("/mobile", mobile::mobile_router())
        .nest("/blockchain", blockchain::blockchain_router())
        .nest("/abac", abac::abac_router())
        .nest("/upgrade", upgrade::upgrade_router())
        .nest("/code-sandbox", code_sandbox::code_sandbox_router())
        .nest("/ai", ai_assistant::ai_router())
        .nest("/analytics", analytics::analytics_router())
        .nest("/compliance", compliance::compliance_router())
        .nest("/employer", employer::employer_router())
        .nest("/library", library::library_router())
        .nest("/gamification", gamification::create_router())
        .nest("/automation", automation::create_router())
        .nest("/developer", developer::developer_router())
        .nest("/iot", iot::iot_router())
        .nest("/iot-advanced", iot_advanced::create_iot_advanced_routes(
            iot_advanced::IotAdvancedState {
                mqtt_client: None,
                edge_manager: std::sync::Arc::new(tokio::sync::RwLock::new(
                    crate::services::edge_computing::EdgeComputingManager::new()
                )),
                maintenance_manager: std::sync::Arc::new(tokio::sync::RwLock::new(
                    crate::services::predictive_maintenance::PredictiveMaintenanceManager::new()
                )),
            }
        ))
        .nest("/ws", websocket::create_websocket_routes(
            websocket::WebSocketState {
                manager: std::sync::Arc::new(crate::services::websocket::WebSocketManager::new()),
            }
        ))
        // Phase 16 & 17 Enhancements
        .nest("/vpat", vpat::vpat_router())
        .nest("/oauth", oauth::oauth_router())
        .nest("/marketplace", marketplace::marketplace_router())
        .nest("/sdk", sdk::sdk_router())
        .nest("/api-analytics", api_analytics::api_analytics_router())
        // Module 18, 23, 24
        .nest("/parents", parents_alumni::parents_router())
        .nest("/id-cards", parents_alumni::id_cards_router())
        .nest("/alumni", parents_alumni::alumni_router())
    // .nest("/users", users::users_router())
}
