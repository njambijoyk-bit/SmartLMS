// Phase 17 Enhancement: OAuth 2.0 Server
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthApplication {
    pub id: Uuid,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub token_type: String,
}

pub struct OAuthService;
impl OAuthService {
    pub fn create_application(name: String, redirect_uris: Vec<String>) -> OAuthApplication {
        OAuthApplication {
            id: Uuid::new_v4(),
            client_id: format!("client_{}", Uuid::new_v4()),
            client_secret: format!("secret_{}", Uuid::new_v4()),
            redirect_uris,
            scopes: vec!["read".to_string(), "write".to_string()],
        }
    }
    
    pub fn generate_token(client_id: String, scopes: Vec<String>) -> OAuthToken {
        OAuthToken {
            access_token: format!("Bearer_{}", Uuid::new_v4()),
            refresh_token: Some(format!("Refresh_{}", Uuid::new_v4())),
            expires_in: 3600,
            token_type: "Bearer".to_string(),
        }
    }
}
