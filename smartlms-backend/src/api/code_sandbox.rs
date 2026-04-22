// Code sandbox API - executes user-supplied code in isolated containers.
//
// The sandbox service itself is expensive to spin up (it holds a handle to
// active tokio child processes), so handlers receive it via Extension rather
// than State. Tests that don't need the sandbox can simply not inject it.

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::code_sandbox::{CodeSandboxService, ExecutionRequest, ExecutionStatus};

pub type SharedSandbox = Arc<CodeSandboxService>;

pub fn code_sandbox_router() -> Router {
    Router::new()
        .route("/execute", post(execute_code))
        .route("/languages", get(get_supported_languages))
        .route("/stop/:execution_id", post(stop_execution))
}

#[derive(Deserialize)]
pub struct ExecuteCodeRequest {
    pub language: String,
    pub code: String,
    #[serde(default)]
    pub input: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
    #[serde(default = "default_memory")]
    pub memory_limit_mb: u64,
    #[serde(default = "default_cpu")]
    pub cpu_limit: f32,
}

fn default_timeout() -> u64 {
    5000
}
fn default_memory() -> u64 {
    128
}
fn default_cpu() -> f32 {
    0.5
}

#[derive(Serialize)]
pub struct ExecuteCodeResponse {
    pub id: String,
    pub status: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub execution_time_ms: u64,
    pub memory_used_kb: u64,
    pub error_message: Option<String>,
}

#[derive(Serialize)]
pub struct LanguageInfo {
    pub name: String,
    pub version: String,
    pub file_extension: String,
}

pub async fn execute_code(
    sandbox: Option<Extension<SharedSandbox>>,
    Json(body): Json<ExecuteCodeRequest>,
) -> impl IntoResponse {
    let Some(Extension(sandbox)) = sandbox else {
        return (StatusCode::SERVICE_UNAVAILABLE, "sandbox not configured").into_response();
    };

    let request = ExecutionRequest {
        language: body.language,
        code: body.code,
        input: body.input,
        timeout_ms: body.timeout_ms,
        memory_limit_mb: body.memory_limit_mb,
        cpu_limit: body.cpu_limit,
    };

    match sandbox.execute(request).await {
        Ok(result) => {
            let response = ExecuteCodeResponse {
                id: result.id,
                status: match result.status {
                    ExecutionStatus::Success => "success".to_string(),
                    ExecutionStatus::RuntimeError => "runtime_error".to_string(),
                    ExecutionStatus::Timeout => "timeout".to_string(),
                    ExecutionStatus::MemoryLimitExceeded => "memory_limit_exceeded".to_string(),
                    ExecutionStatus::CompilationError => "compilation_error".to_string(),
                    ExecutionStatus::InternalError => "internal_error".to_string(),
                },
                stdout: result.stdout,
                stderr: result.stderr,
                exit_code: result.exit_code,
                execution_time_ms: result.execution_time_ms,
                memory_used_kb: result.memory_used_kb,
                error_message: result.error_message,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_supported_languages(
    sandbox: Option<Extension<SharedSandbox>>,
) -> impl IntoResponse {
    let Some(Extension(sandbox)) = sandbox else {
        return (StatusCode::SERVICE_UNAVAILABLE, "sandbox not configured").into_response();
    };

    let languages = sandbox.get_supported_languages().await;
    let language_info: Vec<LanguageInfo> = languages
        .iter()
        .map(|lang| {
            let (version, extension) = match lang.as_str() {
                "python" => ("3.11", "py"),
                "java" => ("17", "java"),
                "cpp" => ("11 (C++17)", "cpp"),
                "c" => ("11 (C11)", "c"),
                "javascript" => ("18", "js"),
                "rust" => ("1.70", "rs"),
                "go" => ("1.21", "go"),
                _ => ("unknown", "txt"),
            };
            LanguageInfo {
                name: lang.clone(),
                version: version.to_string(),
                file_extension: extension.to_string(),
            }
        })
        .collect();

    (StatusCode::OK, Json(language_info)).into_response()
}

pub async fn stop_execution(
    sandbox: Option<Extension<SharedSandbox>>,
    Path(execution_id): Path<String>,
) -> impl IntoResponse {
    let Some(Extension(sandbox)) = sandbox else {
        return (StatusCode::SERVICE_UNAVAILABLE, "sandbox not configured").into_response();
    };

    if sandbox.stop_execution(&execution_id).await {
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Execution stopped successfully",
                "execution_id": execution_id,
            })),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Execution not found or already completed",
            })),
        )
            .into_response()
    }
}
