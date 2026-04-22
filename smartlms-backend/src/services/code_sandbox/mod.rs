use std::collections::HashMap;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::{Command, Child};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("Execution timeout exceeded")]
    Timeout,
    #[error("Memory limit exceeded: {0}MB")]
    MemoryLimit(u64),
    #[error("Compilation failed: {0}")]
    CompilationError(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    #[error("Invalid language: {0}")]
    InvalidLanguage(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Resource limit error: {0}")]
    ResourceLimit(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub language: String,
    pub code: String,
    pub input: Option<String>,
    pub timeout_ms: u64,
    pub memory_limit_mb: u64,
    pub cpu_limit: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub id: String,
    pub status: ExecutionStatus,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub execution_time_ms: u64,
    pub memory_used_kb: u64,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Success,
    RuntimeError,
    Timeout,
    MemoryLimitExceeded,
    CompilationError,
    InternalError,
}

#[derive(Debug, Clone)]
pub struct LanguageConfig {
    pub image_name: String,
    pub compile_command: Option<Vec<String>>,
    pub run_command: Vec<String>,
    pub file_extension: String,
    pub memory_overhead_mb: u64,
}

pub struct CodeSandboxService {
    languages: HashMap<String, LanguageConfig>,
    active_containers: Mutex<HashMap<String, Child>>,
    max_concurrent: usize,
    active_count: Mutex<usize>,
}

impl CodeSandboxService {
    pub fn new() -> Self {
        let mut languages = HashMap::new();
        
        // Python 3.11
        languages.insert("python".to_string(), LanguageConfig {
            image_name: "python:3.11-slim".to_string(),
            compile_command: None,
            run_command: vec!["python3".to_string(), "/code/main.py".to_string()],
            file_extension: "py".to_string(),
            memory_overhead_mb: 50,
        });
        
        // Java 17
        languages.insert("java".to_string(), LanguageConfig {
            image_name: "eclipse-temurin:17-jre-alpine".to_string(),
            compile_command: Some(vec!["javac".to_string(), "-d".to_string(), "/code".to_string(), "/code/Main.java".to_string()]),
            run_command: vec!["java".to_string(), "-cp".to_string(), "/code".to_string(), "Main".to_string()],
            file_extension: "java".to_string(),
            memory_overhead_mb: 128,
        });
        
        // C++ (g++ 11)
        languages.insert("cpp".to_string(), LanguageConfig {
            image_name: "gcc:11-alpine".to_string(),
            compile_command: Some(vec!["g++".to_string(), "-std=c++17".to_string(), "-O2".to_string(), "-o".to_string(), "/code/main".to_string(), "/code/main.cpp".to_string()]),
            run_command: vec!["/code/main".to_string()],
            file_extension: "cpp".to_string(),
            memory_overhead_mb: 30,
        });
        
        // C (gcc 11)
        languages.insert("c".to_string(), LanguageConfig {
            image_name: "gcc:11-alpine".to_string(),
            compile_command: Some(vec!["gcc".to_string(), "-std=c11".to_string(), "-O2".to_string(), "-o".to_string(), "/code/main".to_string(), "/code/main.c".to_string()]),
            run_command: vec!["/code/main".to_string()],
            file_extension: "c".to_string(),
            memory_overhead_mb: 30,
        });
        
        // JavaScript (Node.js 18)
        languages.insert("javascript".to_string(), LanguageConfig {
            image_name: "node:18-alpine".to_string(),
            compile_command: None,
            run_command: vec!["node".to_string(), "/code/main.js".to_string()],
            file_extension: "js".to_string(),
            memory_overhead_mb: 60,
        });
        
        // Rust 1.70
        languages.insert("rust".to_string(), LanguageConfig {
            image_name: "rust:1.70-alpine".to_string(),
            compile_command: Some(vec!["rustc".to_string(), "-o".to_string(), "/code/main".to_string(), "/code/main.rs".to_string()]),
            run_command: vec!["/code/main".to_string()],
            file_extension: "rs".to_string(),
            memory_overhead_mb: 100,
        });
        
        // Go 1.21
        languages.insert("go".to_string(), LanguageConfig {
            image_name: "golang:1.21-alpine".to_string(),
            compile_command: Some(vec!["go".to_string(), "build".to_string(), "-o".to_string(), "/code/main".to_string(), "/code/main.go".to_string()]),
            run_command: vec!["/code/main".to_string()],
            file_extension: "go".to_string(),
            memory_overhead_mb: 50,
        });
        
        Self {
            languages,
            active_containers: Mutex::new(HashMap::new()),
            max_concurrent: 50,
            active_count: Mutex::new(0),
        }
    }
    
    pub async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResult, SandboxError> {
        let execution_id = Uuid::new_v4().to_string();
        let start_time = std::time::Instant::now();
        
        // Check concurrent execution limit
        {
            let count = self.active_count.lock().await;
            if *count >= self.max_concurrent {
                return Ok(ExecutionResult {
                    id: execution_id,
                    status: ExecutionStatus::InternalError,
                    stdout: String::new(),
                    stderr: String::new(),
                    exit_code: None,
                    execution_time_ms: 0,
                    memory_used_kb: 0,
                    error_message: Some("Server busy - too many concurrent executions".to_string()),
                });
            }
        }
        
        // Validate language
        let config = self.languages.get(&request.language)
            .ok_or_else(|| SandboxError::InvalidLanguage(request.language.clone()))?
            .clone();
        
        // Increment active count
        *self.active_count.lock().await += 1;
        
        let result = self.execute_internal(execution_id.clone(), request, config).await;
        
        // Decrement active count
        *self.active_count.lock().await -= 1;
        
        result
    }
    
    async fn execute_internal(
        &self,
        execution_id: String,
        request: ExecutionRequest,
        config: LanguageConfig,
    ) -> Result<ExecutionResult, SandboxError> {
        let start_time = std::time::Instant::now();
        let timeout = Duration::from_millis(request.timeout_ms);
        let effective_memory_limit = request.memory_limit_mb.saturating_sub(config.memory_overhead_mb);
        
        // Create temporary directory for code
        let temp_dir = format!("/tmp/sandbox_{}", execution_id);
        tokio::fs::create_dir_all(&temp_dir).await?;
        
        let filename = format!("main.{}", config.file_extension);
        let filepath = format!("{}/{}", temp_dir, filename);
        
        // Write code to file
        tokio::fs::write(&filepath, &request.code).await?;
        
        // Build Docker command
        let mut docker_args = vec![
            "run".to_string(),
            "--rm".to_string(),
            "--network".to_string(), "none".to_string(),
            "--memory".to_string(), format!("{}m", effective_memory_limit),
            "--cpus".to_string(), request.cpu_limit.to_string(),
            "--pids-limit".to_string(), "50".to_string(),
            "--ulimit".to_string(), "nofile=128:128".to_string(),
            "--ulimit".to_string(), "core=0:0".to_string(),
            "-v".to_string(), format!("{}:/code:ro", temp_dir),
            "-w".to_string(), "/code".to_string(),
        ];
        
        // Add timeout via docker exec timeout
        docker_args.push("--timeout".to_string());
        docker_args.push(format!("{}s", request.timeout_ms / 1000 + 5));
        
        docker_args.push(config.image_name.clone());
        
        // Compilation step if needed
        if let Some(compile_cmd) = &config.compile_command {
            let mut compile_args = docker_args.clone();
            compile_args.extend(compile_cmd.iter().cloned());
            
            let compile_output = Command::new("docker")
                .args(&compile_args)
                .output()
                .await?;
            
            if !compile_output.status.success() {
                let stderr = String::from_utf8_lossy(&compile_output.stderr).to_string();
                return Ok(ExecutionResult {
                    id: execution_id,
                    status: ExecutionStatus::CompilationError,
                    stdout: String::new(),
                    stderr,
                    exit_code: compile_output.status.code(),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                    memory_used_kb: 0,
                    error_message: Some("Compilation failed".to_string()),
                });
            }
        }
        
        // Execution step
        let mut run_args = docker_args;
        run_args.extend(config.run_command.iter().cloned());
        
        let mut child = Command::new("docker")
            .args(&run_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        
        // Track active container
        self.active_containers.lock().await.insert(execution_id.clone(), child);
        
        // Provide input if any
        if let Some(input) = &request.input {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes()).await?;
            }
        }
        
        // Capture output with timeout
        let stdout_handle = tokio::spawn(async move {
            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            let mut output = String::new();
            
            while let Ok(Some(line)) = lines.next_line().await {
                output.push_str(&line);
                output.push('\n');
            }
            
            output
        });
        
        let stderr_handle = tokio::spawn(async move {
            let stderr = child.stderr.take().unwrap();
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            let mut output = String::new();
            
            while let Ok(Some(line)) = lines.next_line().await {
                output.push_str(&line);
                output.push('\n');
            }
            
            output
        });
        
        // Wait for completion with timeout
        let status = tokio::time::timeout(timeout, child.wait()).await;
        
        // Remove from active containers
        self.active_containers.lock().await.remove(&execution_id);
        
        // Cleanup temp directory
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
        
        match status {
            Ok(Ok(exit_status)) => {
                let stdout = stdout_handle.await.unwrap_or_default();
                let stderr = stderr_handle.await.unwrap_or_default();
                let execution_time = start_time.elapsed().as_millis() as u64;
                
                if exit_status.success() {
                    Ok(ExecutionResult {
                        id: execution_id,
                        status: ExecutionStatus::Success,
                        stdout,
                        stderr,
                        exit_code: exit_status.code(),
                        execution_time_ms: execution_time,
                        memory_used_kb: 0, // Would need cgroup stats for accurate measurement
                        error_message: None,
                    })
                } else {
                    Ok(ExecutionResult {
                        id: execution_id,
                        status: ExecutionStatus::RuntimeError,
                        stdout,
                        stderr,
                        exit_code: exit_status.code(),
                        execution_time_ms: execution_time,
                        memory_used_kb: 0,
                        error_message: Some("Runtime error occurred".to_string()),
                    })
                }
            }
            Ok(Err(e)) => {
                Ok(ExecutionResult {
                    id: execution_id,
                    status: ExecutionStatus::InternalError,
                    stdout: String::new(),
                    stderr: e.to_string(),
                    exit_code: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                    memory_used_kb: 0,
                    error_message: Some("Failed to wait for process".to_string()),
                })
            }
            Err(_) => {
                // Timeout - kill the container
                if let Some(mut child) = self.active_containers.lock().await.remove(&execution_id) {
                    let _ = child.kill().await;
                }
                
                Ok(ExecutionResult {
                    id: execution_id,
                    status: ExecutionStatus::Timeout,
                    stdout: String::new(),
                    stderr: String::new(),
                    exit_code: None,
                    execution_time_ms: request.timeout_ms,
                    memory_used_kb: 0,
                    error_message: Some(format!("Execution exceeded {}ms timeout", request.timeout_ms)),
                })
            }
        }
    }
    
    pub async fn get_supported_languages(&self) -> Vec<String> {
        self.languages.keys().cloned().collect()
    }
    
    pub async fn stop_execution(&self, execution_id: &str) -> bool {
        if let Some(mut child) = self.active_containers.lock().await.remove(execution_id) {
            child.kill().await.is_ok()
        } else {
            false
        }
    }
}

impl Default for CodeSandboxService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_python_hello_world() {
        let sandbox = CodeSandboxService::new();
        let request = ExecutionRequest {
            language: "python".to_string(),
            code: "print('Hello, World!')".to_string(),
            input: None,
            timeout_ms: 5000,
            memory_limit_mb: 128,
            cpu_limit: 0.5,
        };
        
        let result = sandbox.execute(request).await.unwrap();
        assert_eq!(result.status, ExecutionStatus::Success);
        assert!(result.stdout.contains("Hello, World!"));
    }
    
    #[tokio::test]
    async fn test_timeout() {
        let sandbox = CodeSandboxService::new();
        let request = ExecutionRequest {
            language: "python".to_string(),
            code: "import time\ntime.sleep(10)".to_string(),
            input: None,
            timeout_ms: 100,
            memory_limit_mb: 128,
            cpu_limit: 0.5,
        };
        
        let result = sandbox.execute(request).await.unwrap();
        assert_eq!(result.status, ExecutionStatus::Timeout);
    }
}
