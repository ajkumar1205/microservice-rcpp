use crate::api::{CodeRequest, CodeResponse, Language};
use log::{debug, error, info};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::{Builder, TempDir};
use tokio::sync::Mutex;
use tonic::{Response, Status};

struct CompilerService {
    temp_dir: Arc<TempDir>,
}

impl CompilerService {
    fn new() -> Result<Self, Status> {
        let temp_dir = Builder::new()
            .prefix("compiler-service-")
            .tempdir()
            .map_err(|e| Status::internal(format!("Failed to create temp directory: {}", e)))?;

        Ok(CompilerService {
            temp_dir: Arc::new(temp_dir),
        })
    }

    fn get_file_path(&self, lang: Language) -> PathBuf {
        let filename = match lang {
            Language::C => "temp.c",
            Language::Cpp => "temp.cpp",
            Language::Rust => "temp.rs",
        };
        self.temp_dir.path().join(filename)
    }

    fn get_input_path(&self) -> PathBuf {
        self.temp_dir.path().join("input.txt")
    }

    fn get_executable_path(&self) -> PathBuf {
        self.temp_dir.path().join("program")
    }
}

// Global service instance wrapped in Arc<Mutex>
lazy_static::lazy_static! {
    static ref COMPILER_SERVICE: Arc<Mutex<Result<CompilerService, Status>>> =
        Arc::new(Mutex::new(CompilerService::new()));
}

pub async fn write_in_file(request: &CodeRequest) -> Result<(), Status> {
    info!("Acquiring lock on COMPILER_SERVICE");
    let service = COMPILER_SERVICE.lock().await;
    let service = service.as_ref().map_err(|e| e.clone())?;
    info!("Lock acquired");

    let file_path = service.get_file_path(request.lang());
    info!("Writing code to file: {:?}", file_path);

    // Format the code properly with newlines
    let formatted_code = request.code.replace(";", ";\n");

    std::fs::write(&file_path, &formatted_code).map_err(|e| {
        error!("Failed to write source file: {}", e);
        Status::internal(format!("Failed to write source file: {}", e))
    })?;

    info!("Code written to file successfully");
    Ok(())
}

pub async fn compile(lang: Language) -> Result<(), Status> {
    info!("Acquiring lock on COMPILER_SERVICE");
    let service = COMPILER_SERVICE.lock().await;
    let service = service.as_ref().map_err(|e| e.clone())?;
    info!("Lock acquired");

    let source_path = service.get_file_path(lang);
    let output_path = service.get_executable_path();
    info!("Compiling source file: {:?}", source_path);

    let output = match lang {
        Language::C => std::process::Command::new("gcc")
            .arg(&source_path)
            .arg("-o")
            .arg(&output_path)
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .output(),
        Language::Cpp => std::process::Command::new("g++")
            .arg(&source_path)
            .arg("-o")
            .arg(&output_path)
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .output(),
        Language::Rust => std::process::Command::new("rustc")
            .arg(&source_path)
            .arg("-o")
            .arg(&output_path)
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .output(),
    }
    .map_err(|e| {
        error!("Compilation failed: {}", e);
        Status::internal(format!("Compilation failed: {}", e))
    })?;

    if !output.status.success() {
        error!(
            "Compilation error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(Status::internal(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    info!("Compilation successful");
    Ok(())
}

pub async fn execute(request: CodeRequest) -> Result<Response<CodeResponse>, Status> {
    info!("Starting execution process");

    // First write the code to file
    write_in_file(&request).await?;
    info!("Code written to file");

    // Then compile it
    compile(request.lang()).await?;
    info!("Code compiled successfully");

    let service = COMPILER_SERVICE.lock().await;
    let service = service.as_ref().map_err(|e| e.clone())?;
    info!("Lock acquired");

    // Write input to file if it's not empty
    if !request.input.is_empty() {
        let input_path = service.get_input_path();
        info!("Writing input to file: {:?}", input_path);
        std::fs::write(&input_path, &request.input).map_err(|e| {
            error!("Failed to write input file: {}", e);
            Status::internal(format!("Failed to write input file: {}", e))
        })?;
        info!("Input written to file successfully");
    }

    let executable_path = service.get_executable_path();
    info!("Executing file: {:?}", executable_path);

    let start = std::time::Instant::now();
    let mut command = std::process::Command::new(&executable_path);
    command.current_dir(service.temp_dir.path());

    // Only set up stdin redirection if there's input
    if !request.input.is_empty() {
        let input_path = service.get_input_path();
        command.stdin(std::process::Stdio::from(
            std::fs::File::open(&input_path).map_err(|e| {
                error!("Failed to open input file: {}", e);
                Status::internal(format!("Failed to open input file: {}", e))
            })?,
        ));
    }

    let output = command
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()
        .map_err(|e| {
            error!("Execution failed: {}", e);
            Status::internal(format!("Execution failed: {}", e))
        })?;

    let duration = start.elapsed();
    info!("Execution time: {} nanoseconds", duration.as_nanos());

    if output.status.success() {
        info!("Execution successful");
        Ok(Response::new(CodeResponse {
            body: String::from_utf8_lossy(&output.stdout).to_string(),
            time: duration.as_nanos() as u64,
        }))
    } else {
        error!(
            "Execution error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Err(Status::unknown(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}
