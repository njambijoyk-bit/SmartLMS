// Phase 17 Enhancement: OAuth 2.0 Authorization Server
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientType { Confidential, Public }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplicationType { Web, Native, SPA, Mobile }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantType { 
    AuthorizationCode, 
    Implicit, 
    ClientCredentials, 
    RefreshToken, 
    Password,
    DeviceCode,
}

impl GrantType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GrantType::AuthorizationCode => "authorization_code",
            GrantType::Implicit => "implicit",
            GrantType::ClientCredentials => "client_credentials",
            GrantType::RefreshToken => "refresh_token",
            GrantType::Password => "password",
            GrantType::DeviceCode => "urn:ietf:params:oauth:grant-type:device_code",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthApplication {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub client_id: String,
    pub client_secret_hash: String,
    pub client_type: ClientType,
    pub application_type: ApplicationType,
    pub owner_id: Uuid,
    pub redirect_uris: Vec<String>,
    pub post_logout_redirect_uris: Vec<String>,
    pub allowed_origins: Vec<String>,
    pub scopes: Vec<String>,
    pub grant_types: Vec<GrantType>,
    pub response_types: Vec<String>,
    pub token_endpoint_auth_method: String,
    pub access_token_lifetime: i64, // seconds
    pub refresh_token_lifetime: i64, // seconds
    pub require_pkce: bool,
    pub require_consent: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationCode {
    pub id: Uuid,
    pub application_id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
    pub nonce: Option<String>,
    pub state: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub id: Uuid,
    pub application_id: Uuid,
    pub user_id: Option<Uuid>,
    pub token_hash: String,
    pub refresh_token_hash: Option<String>,
    pub scopes: Vec<String>,
    pub audience: Vec<String>,
    pub issuer: String,
    pub subject: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: Option<DateTime<Utc>>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub id_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentGrant {
    pub id: Uuid,
    pub application_id: Uuid,
    pub user_id: Uuid,
    pub scopes: Vec<String>,
    pub was_consent_given: bool,
    pub remember_consent: bool,
    pub consent_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCode {
    pub id: Uuid,
    pub application_id: Uuid,
    pub device_code: String,
    pub user_code: String, // Short code for user entry (e.g., "ABCD-1234")
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub interval_seconds: i64,
    pub is_authorized: bool,
    pub authorized_user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

pub struct OAuthService {
    applications: HashMap<Uuid, OAuthApplication>,
    authorization_codes: HashMap<String, AuthorizationCode>,
    access_tokens: HashMap<String, AccessToken>,
    consent_grants: HashMap<(Uuid, Uuid), ConsentGrant>,
    device_codes: HashMap<String, DeviceCode>,
}

impl OAuthService {
    pub fn new() -> Self {
        OAuthService {
            applications: HashMap::new(),
            authorization_codes: HashMap::new(),
            access_tokens: HashMap::new(),
            consent_grants: HashMap::new(),
            device_codes: HashMap::new(),
        }
    }
    
    /// Register a new OAuth application
    pub fn register_application(
        &mut self,
        name: String,
        owner_id: Uuid,
        redirect_uris: Vec<String>,
        application_type: ApplicationType,
        scopes: Vec<String>,
    ) -> OAuthApplication {
        let client_id = format!("oauth_{}", Uuid::new_v4().simple());
        let client_secret = Uuid::new_v4().to_string();
        
        let mut grant_types = vec![GrantType::AuthorizationCode, GrantType::RefreshToken];
        if application_type == ApplicationType::SPA || application_type == ApplicationType::Native {
            grant_types.push(GrantType::ClientCredentials);
        }
        
        let app = OAuthApplication {
            id: Uuid::new_v4(),
            name,
            description: None,
            client_id: client_id.clone(),
            client_secret_hash: format!("hashed_{}", client_secret), // In production, use proper hashing
            client_type: if application_type == ApplicationType::SPA { ClientType::Public } else { ClientType::Confidential },
            application_type,
            owner_id,
            redirect_uris: redirect_uris.clone(),
            post_logout_redirect_uris: vec![],
            allowed_origins: vec![],
            scopes: scopes.clone(),
            grant_types,
            response_types: vec!["code".to_string()],
            token_endpoint_auth_method: "client_secret_basic".to_string(),
            access_token_lifetime: 3600,
            refresh_token_lifetime: 604800, // 7 days
            require_pkce: application_type == ApplicationType::SPA || application_type == ApplicationType::Native,
            require_consent: true,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_used_at: None,
        };
        
        self.applications.insert(app.id, app.clone());
        app
    }
    
    /// Validate client credentials
    pub fn validate_client(&self, client_id: &str, client_secret: &str) -> Option<&OAuthApplication> {
        self.applications.values()
            .find(|app| app.client_id == client_id && 
                      app.client_secret_hash == format!("hashed_{}", client_secret) &&
                      app.is_active)
    }
    
    /// Generate authorization code for Authorization Code flow
    pub fn generate_authorization_code(
        &mut self,
        application_id: Uuid,
        user_id: Uuid,
        scopes: Vec<String>,
        redirect_uri: String,
        code_challenge: Option<String>,
        code_challenge_method: Option<String>,
    ) -> Result<AuthorizationCode, String> {
        let app = self.applications.get(&application_id)
            .ok_or("Application not found")?;
        
        if !app.redirect_uris.contains(&redirect_uri) {
            return Err("Invalid redirect URI".to_string());
        }
        
        let code = format!("auth_{}_{}", Uuid::new_v4().simple(), Utc::now().timestamp());
        
        let auth_code = AuthorizationCode {
            id: Uuid::new_v4(),
            application_id,
            user_id,
            code: code.clone(),
            scopes,
            redirect_uri,
            code_challenge,
            code_challenge_method,
            nonce: None,
            state: None,
            expires_at: Utc::now() + Duration::minutes(10),
            used: false,
            created_at: Utc::now(),
        };
        
        self.authorization_codes.insert(code.clone(), auth_code.clone());
        Ok(auth_code)
    }
    
    /// Exchange authorization code for access token
    pub fn exchange_code_for_token(
        &mut self,
        code: &str,
        client_id: &str,
        redirect_uri: &str,
        code_verifier: Option<&str>,
    ) -> Result<OAuthToken, String> {
        let auth_code = self.authorization_codes.get_mut(code)
            .ok_or("Invalid authorization code")?;
        
        if auth_code.used {
            return Err("Authorization code already used".to_string());
        }
        
        if Utc::now() > auth_code.expires_at {
            return Err("Authorization code expired".to_string());
        }
        
        let app = self.applications.get(&auth_code.application_id)
            .ok_or("Application not found")?;
        
        if app.client_id != client_id {
            return Err("Client ID mismatch".to_string());
        }
        
        if app.redirect_uris.contains(&redirect_uri.to_string()) {
            return Err("Redirect URI mismatch".to_string());
        }
        
        // Verify PKCE if required
        if app.require_pkce {
            if let (Some(challenge), Some(verifier)) = (&auth_code.code_challenge, code_verifier) {
                // In production, verify code_verifier against code_challenge
                let _ = (challenge, verifier);
            } else {
                return Err("PKCE verification required".to_string());
            }
        }
        
        auth_code.used = true;
        
        // Generate access token
        self.generate_access_token(
            auth_code.application_id,
            Some(auth_code.user_id),
            auth_code.scopes.clone(),
        )
    }
    
    /// Generate access token
    pub fn generate_access_token(
        &mut self,
        application_id: Uuid,
        user_id: Option<Uuid>,
        scopes: Vec<String>,
    ) -> Result<OAuthToken, String> {
        let app = self.applications.get(&application_id)
            .ok_or("Application not found")?;
        
        let token = Uuid::new_v4().to_string();
        let refresh_token = Uuid::new_v4().to_string();
        
        let access_token = AccessToken {
            id: Uuid::new_v4(),
            application_id,
            user_id,
            token_hash: format!("hashed_{}", token),
            refresh_token_hash: Some(format!("hashed_{}", refresh_token)),
            scopes: scopes.clone(),
            audience: vec![],
            issuer: "smartlms".to_string(),
            subject: user_id.map(|id| id.to_string()),
            expires_at: Utc::now() + Duration::seconds(app.access_token_lifetime),
            refresh_expires_at: Some(Utc::now() + Duration::seconds(app.refresh_token_lifetime)),
            is_revoked: false,
            created_at: Utc::now(),
        };
        
        self.access_tokens.insert(token.clone(), access_token);
        
        Ok(OAuthToken {
            access_token: format!("slm_{}", token),
            token_type: "Bearer".to_string(),
            expires_in: app.access_token_lifetime,
            refresh_token: Some(format!("slm_refresh_{}", refresh_token)),
            scope: scopes.join(" "),
            id_token: None,
        })
    }
    
    /// Validate access token
    pub fn validate_token(&self, token: &str) -> Option<&AccessToken> {
        self.access_tokens.get(token.strip_prefix("slm_").unwrap_or(token))
            .filter(|t| !t.is_revoked && Utc::now() < t.expires_at)
    }
    
    /// Revoke access token
    pub fn revoke_token(&mut self, token: &str) -> bool {
        if let Some(access_token) = self.access_tokens.get_mut(token.strip_prefix("slm_").unwrap_or(token)) {
            access_token.is_revoked = true;
            true
        } else {
            false
        }
    }
    
    /// Check if user has granted consent to application
    pub fn check_consent(&self, user_id: Uuid, application_id: Uuid) -> Option<&ConsentGrant> {
        self.consent_grants.get(&(user_id, application_id))
            .filter(|grant| grant.was_consent_given && 
                   grant.consent_expires_at.map_or(true, |exp| Utc::now() < exp))
    }
    
    /// Record user consent
    pub fn record_consent(
        &mut self,
        user_id: Uuid,
        application_id: Uuid,
        scopes: Vec<String>,
        remember: bool,
    ) -> ConsentGrant {
        let consent_expires_at = if remember {
            Some(Utc::now() + Duration::days(30))
        } else {
            None
        };
        
        let grant = ConsentGrant {
            id: Uuid::new_v4(),
            application_id,
            user_id,
            scopes,
            was_consent_given: true,
            remember_consent: remember,
            consent_expires_at,
            created_at: Utc::now(),
        };
        
        self.consent_grants.insert((user_id, application_id), grant.clone());
        grant
    }
    
    /// Initiate device flow
    pub fn initiate_device_flow(
        &mut self,
        application_id: Uuid,
        scopes: Vec<String>,
    ) -> Result<DeviceCode, String> {
        let app = self.applications.get(&application_id)
            .ok_or("Application not found")?;
        
        let device_code = format!("device_{}", Uuid::new_v4().simple());
        let user_code = Self::generate_user_code();
        
        let device = DeviceCode {
            id: Uuid::new_v4(),
            application_id,
            device_code: device_code.clone(),
            user_code: user_code.clone(),
            verification_uri: "https://smartlms.com/oauth/device".to_string(),
            verification_uri_complete: Some(format!(
                "https://smartlms.com/oauth/device?user_code={}",
                user_code
            )),
            scopes,
            expires_at: Utc::now() + Duration::minutes(15),
            interval_seconds: 5,
            is_authorized: false,
            authorized_user_id: None,
            created_at: Utc::now(),
        };
        
        self.device_codes.insert(device_code.clone(), device.clone());
        Ok(device)
    }
    
    /// Generate human-readable user code for device flow
    fn generate_user_code() -> String {
        let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
        let mut code = String::new();
        for i in 0..8 {
            if i == 4 {
                code.push('-');
            }
            let idx = (i as usize) % chars.len();
            code.push(chars[idx]);
        }
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_application() {
        let mut service = OAuthService::new();
        let app = service.register_application(
            "Test App".to_string(),
            Uuid::new_v4(),
            vec!["https://example.com/callback".to_string()],
            ApplicationType::Web,
            vec!["read".to_string(), "write".to_string()],
        );
        
        assert_eq!(app.name, "Test App");
        assert!(app.is_active);
        assert_eq!(app.application_type, ApplicationType::Web);
    }
    
    #[test]
    fn test_device_code_generation() {
        let user_code = OAuthService::generate_user_code();
        assert_eq!(user_code.len(), 9); // 4 chars + dash + 4 chars
        assert_eq!(user_code.chars().nth(4), Some('-'));
    }
}
