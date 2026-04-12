// Phase 17 Enhancement: OAuth 2.0 Authorization Server API
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Redirect,
    Form,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::services::oauth_server::{
    OAuthService, 
    AuthorizationRequest, 
    TokenRequest, 
    TokenResponse, 
    AuthorizationCodeResponse,
    ConsentDecision,
    DeviceAuthorizationRequest,
    DeviceAuthorizationResponse,
};
use crate::utils::app_state::AppState;

#[derive(Debug, Deserialize)]
pub struct AuthorizeQuery {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConsentForm {
    pub client_id: String,
    pub user_id: String,
    pub decision: String, // "allow" or "deny"
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenForm {
    pub grant_type: String,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    // PKCE parameters
    pub code_verifier: Option<String>,
    // Device flow
    pub device_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OAuthError {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WellKnownConfig {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
    pub response_types_supported: Vec<String>,
    pub grant_types_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
}

/// GET /api/oauth/authorize - OAuth 2.0 authorization endpoint
pub async fn authorize(
    State(state): State<AppState>,
    Query(params): Query<AuthorizeQuery>,
) -> Result<Redirect, (StatusCode, Json<OAuthError>)> {
    if params.response_type != "code" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(OAuthError {
                error: "unsupported_response_type".to_string(),
                error_description: Some("Only authorization code flow is supported".to_string()),
            }),
        ));
    }

    let auth_request = AuthorizationRequest {
        client_id: params.client_id,
        redirect_uri: params.redirect_uri.unwrap_or_default(),
        scope: params.scope.unwrap_or_default(),
        state: params.state,
        code_challenge: params.code_challenge,
        code_challenge_method: params.code_challenge_method,
    };

    // Validate request and create authorization code
    match OAuthService::validate_authorization_request(&auth_request) {
        Ok(_) => {
            // Store authorization request in session/state
            // For now, redirect to consent page
            let consent_url = format!(
                "/oauth/consent?client_id={}&scope={}&state={}",
                params.client_id,
                params.scope.unwrap_or_default(),
                params.state.unwrap_or_default()
            );
            Ok(Redirect::to(&consent_url))
        }
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(OAuthError {
                error: "invalid_request".to_string(),
                error_description: Some(e),
            }),
        )),
    }
}

/// POST /api/oauth/token - OAuth 2.0 token endpoint
pub async fn token(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(payload): Form<TokenForm>,
) -> Result<Json<TokenResponse>, (StatusCode, Json<OAuthError>)> {
    let token_request = TokenRequest {
        grant_type: payload.grant_type,
        code: payload.code,
        redirect_uri: payload.redirect_uri,
        client_id: payload.client_id,
        client_secret: payload.client_secret,
        refresh_token: payload.refresh_token,
        scope: payload.scope,
        code_verifier: payload.code_verifier,
        device_code: payload.device_code,
    };

    match OAuthService::handle_token_request(token_request) {
        Ok(token_response) => Ok(Json(token_response)),
        Err(e) => {
            let (error_code, description) = match e.as_str() {
                "invalid_grant" => ("invalid_grant", "Invalid authorization code or refresh token"),
                "invalid_client" => ("invalid_client", "Client authentication failed"),
                "unauthorized_client" => ("unauthorized_client", "Client is not authorized for this grant type"),
                "invalid_scope" => ("invalid_scope", "Invalid scope requested"),
                _ => ("server_error", "Internal server error"),
            };
            
            Err((
                StatusCode::BAD_REQUEST,
                Json(OAuthError {
                    error: error_code.to_string(),
                    error_description: Some(description.to_string()),
                }),
            ))
        }
    }
}

/// GET /api/oauth/consent - Display OAuth consent page
pub async fn consent_page(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<String, StatusCode> {
    let client_id = params.get("client_id").ok_or(StatusCode::BAD_REQUEST)?;
    let scope = params.get("scope").map(|s| s.as_str()).unwrap_or("");
    
    // TODO: Fetch client details and render consent HTML
    // For now, return a simple HTML form
    Ok(format!(
        r#"<!DOCTYPE html>
<html>
<head><title>OAuth Consent</title></head>
<body>
    <h1>Authorize Application</h1>
    <p>Client ID: {}</p>
    <p>Requested Scopes: {}</p>
    <form method="POST" action="/api/oauth/consent">
        <input type="hidden" name="client_id" value="{}" />
        <input type="hidden" name="user_id" value="current_user" />
        <button type="submit" name="decision" value="allow">Allow</button>
        <button type="submit" name="decision" value="deny">Deny</button>
    </form>
</body>
</html>"#,
        client_id, scope, client_id
    ))
}

/// POST /api/oauth/consent - Handle OAuth consent decision
pub async fn handle_consent(
    State(state): State<AppState>,
    Form(payload): Form<ConsentForm>,
) -> Result<Redirect, StatusCode> {
    let decision = match payload.decision.as_str() {
        "allow" => ConsentDecision::Allow,
        "deny" => ConsentDecision::Deny,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match OAuthService::process_consent(
        &payload.client_id,
        &payload.user_id,
        decision,
        &payload.scopes,
    ) {
        Ok(redirect_uri) => Ok(Redirect::to(&redirect_uri)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// POST /api/oauth/device/authorize - Device Authorization Endpoint (RFC 8628)
pub async fn device_authorize(
    State(state): State<AppState>,
    Form(payload): Form<HashMap<String, String>>,
) -> Result<Json<DeviceAuthorizationResponse>, (StatusCode, Json<OAuthError>)> {
    let client_id = payload.get("client_id").ok_or((
        StatusCode::BAD_REQUEST,
        Json(OAuthError {
            error: "invalid_request".to_string(),
            error_description: Some("client_id is required".to_string()),
        }),
    ))?;

    let scope = payload.get("scope").map(|s| s.as_str()).unwrap_or("");

    let request = DeviceAuthorizationRequest {
        client_id: client_id.clone(),
        scope: scope.to_string(),
    };

    match OAuthService::device_authorization(request) {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(OAuthError {
                error: "invalid_request".to_string(),
                error_description: Some(e),
            }),
        )),
    }
}

/// POST /api/oauth/revoke - Revoke access or refresh token (RFC 7009)
pub async fn revoke_token(
    State(state): State<AppState>,
    Form(payload): Form<HashMap<String, String>>,
) -> Result<StatusCode, (StatusCode, Json<OAuthError>)> {
    let token = payload.get("token").ok_or((
        StatusCode::BAD_REQUEST,
        Json(OAuthError {
            error: "invalid_request".to_string(),
            error_description: Some("token is required".to_string()),
        }),
    ))?;

    let token_type_hint = payload.get("token_type_hint").map(|s| s.as_str());

    match OAuthService::revoke_token(token, token_type_hint) {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::OK), // Per RFC 7009, always return 200 to prevent enumeration
    }
}

/// GET /api/oauth/.well-known/openid-configuration - OpenID Connect discovery
pub async fn openid_configuration() -> Result<Json<WellKnownConfig>, StatusCode> {
    let config = WellKnownConfig {
        issuer: "https://smartlms.com".to_string(),
        authorization_endpoint: "https://smartlms.com/api/oauth/authorize".to_string(),
        token_endpoint: "https://smartlms.com/api/oauth/token".to_string(),
        jwks_uri: "https://smartlms.com/api/oauth/jwks".to_string(),
        response_types_supported: vec!["code".to_string()],
        grant_types_supported: vec![
            "authorization_code".to_string(),
            "refresh_token".to_string(),
            "urn:ietf:params:oauth:grant-type:device_code".to_string(),
        ],
        token_endpoint_auth_methods_supported: vec![
            "client_secret_basic".to_string(),
            "client_secret_post".to_string(),
        ],
        scopes_supported: vec![
            "openid".to_string(),
            "profile".to_string(),
            "email".to_string(),
            "offline_access".to_string(),
        ],
    };

    Ok(Json(config))
}

/// GET /api/oauth/jwks - JSON Web Key Set endpoint
pub async fn jwks() -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Generate actual JWKS from OAuthService
    let jwks = serde_json::json!({
        "keys": []
    });
    Ok(Json(jwks))
}

/// GET /api/oauth/userinfo - OpenID Connect UserInfo endpoint
pub async fn userinfo(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<OAuthError>)> {
    // Extract Bearer token from Authorization header
    let auth_header = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    let token = auth_header.ok_or((
        StatusCode::UNAUTHORIZED,
        Json(OAuthError {
            error: "invalid_token".to_string(),
            error_description: Some("Missing or invalid Authorization header".to_string()),
        }),
    ))?;

    // TODO: Validate token and return user info
    match OAuthService::get_user_info(token) {
        Ok(user_info) => Ok(Json(user_info)),
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(OAuthError {
                error: "invalid_token".to_string(),
                error_description: Some("Token is invalid or expired".to_string()),
            }),
        )),
    }
}

pub fn oauth_router() -> axum::Router {
    axum::Router::new()
        .route("/authorize", axum::routing::get(authorize))
        .route("/token", axum::routing::post(token))
        .route("/consent", axum::routing::get(consent_page))
        .route("/consent", axum::routing::post(handle_consent))
        .route("/device/authorize", axum::routing::post(device_authorize))
        .route("/revoke", axum::routing::post(revoke_token))
        .route("/userinfo", axum::routing::get(userinfo))
        .route("/jwks", axum::routing::get(jwks))
        .route("/.well-known/openid-configuration", axum::routing::get(openid_configuration))
}
