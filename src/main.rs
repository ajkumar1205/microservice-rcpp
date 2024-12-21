mod utils;
use std::borrow::Borrow;
use log::{info, error};
use api::{
    code_server::{Code, CodeServer},
    CodeRequest, CodeResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug, Default)]
pub struct CodeService {}

#[tonic::async_trait]
impl Code for CodeService {
    async fn post(&self, request: Request<CodeRequest>) -> Result<Response<CodeResponse>, Status> {
        let req = request.into_inner();
        info!("Received request: {:?}", req);

        utils::write_in_file(req.borrow()).await?;
        info!("Code written to file");

        utils::compile(req.lang()).await?;
        info!("Code compiled successfully");

        let response = utils::execute(req).await?;
        info!("Code executed successfully");

        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let adr = "127.0.0.1:50051".parse()?;
    info!("Server is running on {}", adr);
    let code_service = CodeService::default();

    Server::builder()
        .add_service(CodeServer::new(code_service))
        .serve(adr)
        .await?;

    Ok(())
}