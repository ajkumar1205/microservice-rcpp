const BASE_PATH: &str = "/home/ajay/RustProjects/COMPILER_API/c-cpp-rust/data";

use crate::api::{CodeRequest, CodeResponse, Language};
use std::sync::Arc;
use std::{fs::File, io::Write};
use tokio::sync::Mutex;
use tonic::{Response, Status};

pub async fn write_in_file(request: &CodeRequest) -> Result<(), Status> {
    let file: Arc<Mutex<File>>;
    match request.lang() {
        Language::C => {
            file = Arc::new(Mutex::new(
                File::create(format!("{}/c/temp.c", BASE_PATH))
                    .map_err(|_| Status::resource_exhausted("Unable to create temp.c"))?,
            ));
        }
        Language::Cpp => {
            file = Arc::new(Mutex::new(
                File::create(format!("{}/cpp/temp.cpp", BASE_PATH))
                    .map_err(|_| Status::resource_exhausted("Unable to create temp.cpp"))?,
            ));
        }
        Language::Rust => {
            file = Arc::new(Mutex::new(
                File::create(format!("{}/rust/temp.rs", BASE_PATH))
                    .map_err(|_| Status::resource_exhausted("Unable to create temp.rs"))?,
            ));
        }
    }

    file.lock()
        .await
        .write_all(request.code.as_bytes())
        .map_err(|_| Status::resource_exhausted("Unable to save source code in the file"))?;

    Ok(())
}

pub async fn compile(lang: Language) -> Result<(), Status> {
    let output: std::process::Output;
    match lang {
        Language::C => {
            output = std::process::Command::new("gcc")
                .arg("temp.c")
                .current_dir(format!("{}/c", BASE_PATH))
                .output()
                .map_err(|_| Status::resource_exhausted("Unable to compile the code"))?;
        }
        Language::Cpp => {
            output = std::process::Command::new("g++")
                .arg("temp.cpp")
                .current_dir(format!("{}/cpp", BASE_PATH))
                .output()
                .map_err(|_| Status::resource_exhausted("Unable to compile the code"))?;
        }
        Language::Rust => {
            output = std::process::Command::new("rustc")
                .arg("temp.rs")
                .current_dir(format!("{}/rust", BASE_PATH))
                .output()
                .map_err(|_| Status::resource_exhausted("Unable to compile the code"))?;
        }
    }

    if output.stdout.is_empty() {
        return Err(Status::resource_exhausted(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

pub async fn execute(request: CodeRequest) -> Result<Response<CodeResponse>, Status> {
    let file: Arc<Mutex<File>>;

    match request.lang() {
        Language::C => {
            file = Arc::new(Mutex::new(
                File::create(format!("{}/c/in.log", BASE_PATH))
                    .map_err(|_| Status::resource_exhausted("Unable to create in.log"))?,
            ));
        }
        Language::Cpp => {
            file = Arc::new(Mutex::new(
                File::create(format!("{}/cpp/in.log", BASE_PATH))
                    .map_err(|_| Status::resource_exhausted("Unable to create in.log"))?,
            ));
        }
        Language::Rust => {
            file = Arc::new(Mutex::new(
                File::create(format!("{}/rust/in.log", BASE_PATH))
                    .map_err(|_| Status::resource_exhausted("Unable to create in.log"))?,
            ));
        }
    }

    file.lock()
        .await
        .write_all(request.input.as_bytes())
        .map_err(|_| Status::resource_exhausted("Unable to save input in the file"))?;

    let output: std::process::Output;
    let start = std::time::Instant::now();
    match request.lang() {
        Language::C => {
            output = std::process::Command::new("./run.sh")
                .current_dir(format!("{}/c", BASE_PATH))
                .output()
                .map_err(|_| Status::resource_exhausted("Unable to execute the code"))?;
        }
        Language::Cpp => {
            output = std::process::Command::new("./run.sh")
                .current_dir(format!("{}/cpp", BASE_PATH))
                .output()
                .map_err(|_| Status::resource_exhausted("Unable to execute the code"))?;
        }
        Language::Rust => {
            output = std::process::Command::new("./run.sh")
                .current_dir(format!("{}/rust", BASE_PATH))
                .output()
                .map_err(|_| Status::resource_exhausted("Unable to execute the code"))?;
        }
    }
    let duration = start.elapsed();

    if output.stderr.is_empty() {
        return Ok(Response::new(CodeResponse {
            body: String::from_utf8_lossy(&output.stdout).to_string(),
            time: duration.as_nanos() as u64,
        }));
    }

    Err(Status::unknown(String::from_utf8_lossy(&output.stderr)))
}
