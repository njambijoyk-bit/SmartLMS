use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use crate::services::code_sandbox::{CodeSandboxService, ExecutionRequest, ExecutionStatus};

pub fn code_sandbox_router() -> axum::Router {
    use axum::routing::{get, post};
    
    axum::Router::new()
        .route("/execute", post(execute_code))
        .route("/languages", get(get_supported_languages))
        .route("/stop/:execution_id", post(stop_execution))
}

#[derive(Deserialize)]
pub struct ExecuteCodeRequest {
    language: String,
    code: String,
    #[serde(default)]
    input: Option<String>,
    #[serde(default = "default_timeout")]
    timeout_ms: u64,
    #[serde(default = "default_memory")]
    memory_limit_mb: u64,
    #[serde(default = "default_cpu")]
    cpu_limit: f32,
}

fn default_timeout() -> u64 { 5000 }
fn default_memory() -> u64 { 128 }
fn default_cpu() -> f32 { 0.5 }

#[derive(Serialize)]
pub struct ExecuteCodeResponse {
    id: String,
    status: String,
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
    execution_time_ms: u64,
    memory_used_kb: u64,
    error_message: Option<String>,
}

#[derive(Serialize)]
pub struct LanguageInfo {
    name: String,
    version: String,
    file_extension: String,
}

pub async fn execute_code(
    sandbox: web::Data<CodeSandboxService>,
    body: web::Json<ExecuteCodeRequest>,
) -> Result<HttpResponse, Error> {
    let request = ExecutionRequest {
        language: body.language.clone(),
        code: body.code.clone(),
        input: body.input.clone(),
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
            
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().body(e.to_string()))
        }
    }
}

pub async fn get_supported_languages(
    sandbox: web::Data<CodeSandboxService>,
) -> Result<HttpResponse, Error> {
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
    
    Ok(HttpResponse::Ok().json(language_info))
}

pub async fn stop_execution(
    sandbox: web::Data<CodeSandboxService>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let execution_id = path.into_inner();
    
    if sandbox.stop_execution(&execution_id).await {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Execution stopped successfully",
            "execution_id": execution_id
        })))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Execution not found or already completed"
        })))
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/code-sandbox")
            .route("/execute", web::post().to(execute_code))
            .route("/languages", web::get().to(get_supported_languages))
            .route("/stop/{execution_id}", web::post().to(stop_execution))
    );
}
