// SSO Service - Google, Microsoft, SAML integration
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::tenant::InstitutionCtx;

/// SSO provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SsoProvider {
    Google,
    Microsoft,
    SAML,
}

/// SSO user info from provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoUserInfo {
    pub provider: SsoProvider,
    pub provider_id: String,      // Unique ID from provider
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
}

/// SSO configuration for institution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoConfig {
    pub provider: SsoProvider,
    pub enabled: bool,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub tenant_id: Option<String>,        // For Microsoft
    pub organization_id: Option<String>,   // For Google
    pub idp_metadata_url: Option<String>,  // For SAML
    pub sp_entity_id: Option<String>,      // For SAML
    pub attribute_mapping: Option<SsoAttributeMapping>,
}

/// SAML/SSO attribute mapping
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SsoAttributeMapping {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub groups: Option<String>,
}

impl Default for SsoConfig {
    fn default() -> Self {
        Self {
            provider: SsoProvider::Google,
            enabled: false,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            tenant_id: None,
            organization_id: None,
            idp_metadata_url: None,
            sp_entity_id: None,
            attribute_mapping: None,
        }
    }
}

/// Generate OAuth URL for SSO login
pub fn generate_oauth_url(
    config: &SsoConfig,
    state: &str,
    redirect_url: Option<&str>,
) -> Result<String, String> {
    let client_id = config.client_id.as_ref().ok_or("Client ID not configured")?;
    let redirect_uri = config.redirect_uri.as_ref().ok_or("Redirect URI not configured")?;
    
    match config.provider {
        SsoProvider::Google => {
            let mut url = format!(
                "https://accounts.google.com/o/oauth2/v2/auth?\
                client_id={}&\
                redirect_uri={}&\
                response_type=code&\
                scope=email%20profile&\
                state={}",
                client_id, redirect_uri, state
            );
            
            if let Some(org) = &config.organization_id {
                url.push_str(&format!("&hd={}", org));
            }
            
            if let Some(redir) = redirect_url {
                url.push_str(&format!("&state={}", redir));
            }
            
            Ok(url)
        }
        
        SsoProvider::Microsoft => {
            let tenant = config.tenant_id.as_deref().unwrap_or("common");
            let mut url = format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize?\
                client_id={}&\
                redirect_uri={}&\
                response_type=code&\
                scope=email%20profile%20User.Read&\
                state={}",
                tenant, client_id, redirect_uri, state
            );
            
            if let Some(redir) = redirect_url {
                url.push_str(&format!("&redirect_uri={}", redir));
            }
            
            Ok(url)
        }
        
        SsoProvider::SAML => {
            Err("SAML uses XML-based flow, not OAuth URL".to_string())
        }
    }
}

/// Exchange OAuth code for tokens
pub async fn exchange_code(
    config: &SsoConfig,
    code: &str,
) -> Result<SsoTokens, String> {
    let client_id = config.client_id.as_ref().ok_or("Client ID not configured")?;
    let client_secret = config.client_secret.as_ref().ok_or("Client secret not configured")?;
    let redirect_uri = config.redirect_uri.as_ref().ok_or("Redirect URI not configured")?;
    
    match config.provider {
        SsoProvider::Google => {
            let response = reqwest::Client::new()
                .post("https://oauth2.googleapis.com/token")
                .form(&[
                    ("client_id", client_id.as_str()),
                    ("client_secret", client_secret.as_str()),
                    ("code", code),
                    ("grant_type", "authorization_code"),
                    ("redirect_uri", redirect_uri.as_str()),
                ])
                .send()
                .await
                .map_err(|e| e.to_string())?;
            
            let body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
            
            Ok(SsoTokens {
                access_token: body["access_token"].as_str().unwrap_or("").to_string(),
                refresh_token: body["refresh_token"].as_str().unwrap_or("").to_string(),
                expires_in: body["expires_in"].as_i64().unwrap_or(3600),
                id_token: body["id_token"].as_str().unwrap_or("").to_string(),
            })
        }
        
        SsoProvider::Microsoft => {
            let tenant = config.tenant_id.as_deref().unwrap_or("common");
            let response = reqwest::Client::new()
                .post(&format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", tenant))
                .form(&[
                    ("client_id", client_id.as_str()),
                    ("client_secret", client_secret.as_str()),
                    ("code", code),
                    ("grant_type", "authorization_code"),
                    ("redirect_uri", redirect_uri.as_str()),
                    ("scope", "email profile User.Read offline_access"),
                ])
                .send()
                .await
                .map_err(|e| e.to_string())?;
            
            let body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
            
            Ok(SsoTokens {
                access_token: body["access_token"].as_str().unwrap_or("").to_string(),
                refresh_token: body["refresh_token"].as_str().unwrap_or("").to_string(),
                expires_in: body["expires_in"].as_i64().unwrap_or(3600),
                id_token: body["id_token"].as_str().unwrap_or("").to_string(),
            })
        }
        
        SsoProvider::SAML => {
            Err("SAML exchange not implemented".to_string())
        }
    }
}

/// Fetch user info from SSO provider
pub async fn get_user_info(
    config: &SsoConfig,
    tokens: &SsoTokens,
) -> Result<SsoUserInfo, String> {
    match config.provider {
        SsoProvider::Google => {
            let response = reqwest::Client::new()
                .get("https://www.googleapis.com/oauth2/v2/userinfo")
                .header("Authorization", format!("Bearer {}", tokens.access_token))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            
            let body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
            
            Ok(SsoUserInfo {
                provider: SsoProvider::Google,
                provider_id: body["id"].as_str().unwrap_or("").to_string(),
                email: body["email"].as_str().unwrap_or("").to_string(),
                first_name: body["given_name"].as_str().unwrap_or("").to_string(),
                last_name: body["family_name"].as_str().unwrap_or("").to_string(),
                avatar_url: body["picture"].as_str().map(String::from),
            })
        }
        
        SsoProvider::Microsoft => {
            let response = reqwest::Client::new()
                .get("https://graph.microsoft.com/v1.0/me")
                .header("Authorization", format!("Bearer {}", tokens.access_token))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            
            let body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
            
            Ok(SsoUserInfo {
                provider: SsoProvider::Microsoft,
                provider_id: body["id"].as_str().unwrap_or("").to_string(),
                email: body["userPrincipalName"].as_str().unwrap_or("").to_string(),
                first_name: body["givenName"].as_str().unwrap_or("").to_string(),
                last_name: body["surname"].as_str().unwrap_or("").to_string(),
                avatar_url: None,
            })
        }
        
        SsoProvider::SAML => {
            Err("SAML user info not implemented".to_string())
        }
    }
}

/// SSO tokens from OAuth exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub id_token: String,
}

/// Find or create user from SSO
pub async fn find_or_create_sso_user(
    pool: &sqlx::PgPool,
    sso_info: &SsoUserInfo,
) -> Result<User, String> {
    // Check if user exists by email
    if let Some(user) = crate::db::user::find_by_email(pool, &sso_info.email).await.map_err(|e| e.to_string())? {
        // Update SSO provider info if needed
        return Ok(user);
    }
    
    // Create new user
    let user = crate::db::user::create(
        pool,
        &sso_info.email,
        "", // No password for SSO users
        &sso_info.first_name,
        &sso_info.last_name,
        "learner", // Default role
    ).await.map_err(|e| e.to_string())?;
    
    // TODO: Store SSO provider link
    
    Ok(user)
}