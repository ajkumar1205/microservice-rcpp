mod lib;
use lib::c;
use lib::cpp;
use lib::rust;
use tonic::{transport::Server, Request, Response, Status};
use api::{CodeRequest, CodeResponse};
use api::c_code_server::{CCode, CCodeServer};
use api::cpp_code_server::{CppCode, CppCodeServer};
use api::rust_code_server::{RustCode, RustCodeServer};


pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug, Default)]
pub struct CCodeService {}

#[tonic::async_trait]
impl CCode for CCodeService {
    async fn take(&self, request: Request<CodeRequest>) -> Result<Response<CodeResponse>, Status> {
        println!("Request is there {:?}", request);

        let req = request.into_inner();

        let err = c::write_in_file(req.code);

        match err {
            Ok(_) => {}
            Err(e) => {
                let res = CodeResponse{ error: true, body: e.to_string()};
                return Ok(Response::new(res));
            }
        }

        let err = c::compile();

        match err {
            Ok(()) => {}
            Err(e) => {
                let res = CodeResponse{ error: true, body: e.to_string()};
                return Ok(Response::new(res));
            }
        }

        let res = c::execute(req.input);

        Ok(Response::new(CodeResponse {error:false, body: res}))
    }
}



#[derive(Debug, Default)]
pub struct CppCodeService {}

#[tonic::async_trait]
impl CppCode for CppCodeService {
    async fn take(&self, request: Request<CodeRequest>) -> Result<Response<CodeResponse>, Status> {
        println!("Request is there {:?}", request);

        let req = request.into_inner();

        let err = cpp::write_in_file(req.code);

        match err {
            Ok(_) => {}
            Err(e) => {
                let res = CodeResponse{ error: true, body: e.to_string()};
                return Ok(Response::new(res));
            }
        }

        let err = cpp::compile();

        match err {
            Ok(()) => {}
            Err(e) => {
                let res = CodeResponse{ error: true, body: e.to_string()};
                return Ok(Response::new(res));
            }
        }

        let res = cpp::execute(req.input);

        Ok(Response::new(CodeResponse {error:false, body: res}))
    }
}



#[derive(Debug, Default)]
pub struct RustCodeService {}

#[tonic::async_trait]
impl RustCode for RustCodeService {
    async fn take(&self, request: Request<CodeRequest>) -> Result<Response<CodeResponse>, Status> {
        println!("Request is there {:?}", request);

        let req = request.into_inner();

        let err = rust::write_in_file(req.code);

        match err {
            Ok(_) => {}
            Err(e) => {
                let res = CodeResponse{ error: true, body: e.to_string()};
                return Ok(Response::new(res));
            }
        }

        let err = rust::compile();

        match err {
            Ok(()) => {}
            Err(e) => {
                let res = CodeResponse{ error: true, body: e.to_string()};
                return Ok(Response::new(res));
            }
        }

        let res = rust::execute(req.input);

        Ok(Response::new(CodeResponse {error:false, body: res}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let add = "[::1]:50051".parse()?;
    let cser = CCodeService::default();
    let cppser = CppCodeService::default();
    let rser = RustCodeService::default();

    Server::builder()
        .add_service(CCodeServer::new(cser))
        .add_service(CppCodeServer::new(cppser))
        .add_service(RustCodeServer::new(rser))
        .serve(add)
        .await?;

    Ok(())
}
