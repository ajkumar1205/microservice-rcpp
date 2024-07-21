mod utils;
use std::borrow::Borrow;

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

        utils::write_in_file(req.borrow()).await?;
        utils::compile(req.lang()).await?;
        utils::execute(req).await?;

        Err(Status::internal("Some went unexpected"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adr = "127.0.0.1:50051".parse()?;
    println!("Server is running on {}", adr);
    let code_service = CodeService::default();

    Server::builder()
        .add_service(CodeServer::new(code_service))
        .serve(adr)
        .await?;

    Ok(())
}
