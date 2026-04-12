// API module - HTTP route handlers
pub mod abac;
pub mod ai_assistant;
pub mod analytics;
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
pub mod mobile;
pub mod upgrade;
pub mod users;
pub mod developer;

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
    // .nest("/users", users::users_router())
}
