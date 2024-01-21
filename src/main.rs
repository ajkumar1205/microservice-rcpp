use actix_web::post;
use actix_web::{App, HttpServer, get, web::Json};
use serde::{Serialize, Deserialize};



use std::io::Result;


mod lib;
use lib::c;
use lib::cpp;
use lib::rust;

#[derive(Serialize)]
struct Msg {
    error: bool,
    message: String
}

#[derive(Deserialize)]
struct CodeRequest {
    code: String,
    input: String
}

#[get("/")]
async fn index() -> Json<Msg> {
    Json(Msg { error: false, message: "API Endpoints\n\t\"/\" -> For this message\n\t\"/c\" -> For C\n\t\"/cpp\" -> For Cpp\n\"/rust\" -> For Rust".to_string() })
}


#[post("/c")]
async fn c_request(creq: Json<CodeRequest>) -> Json<Msg> {
    if c::write_in_file(creq.code.clone()).is_some() {
        return Json(Msg { error: true, message: "Unable to parse the Code".to_string()});
    }

    let out = c::compile();
    if out.as_ref().is_some_and(|str| str.len()>1) {
        return Json(Msg { error: true, message: out.unwrap()});
    }else {
        let out = c::execute(creq.input.clone());
        return Json(Msg { error: false, message: out});
    }
}

#[post("/cpp")]
async fn cpp_request(creq: Json<CodeRequest>) -> Json<Msg> {
    if cpp::write_in_file(creq.code.clone()).is_some() {
        return Json(Msg { error: true, message: "Unable to parse the Code".to_string()});
    }
    
    let out = cpp::compile();
    if out.as_ref().is_some_and(|str| str.len()>1) {
        return Json(Msg { error: true, message: out.unwrap()});
    }else {
        let out = cpp::execute(creq.input.clone());
        return Json(Msg { error: false, message: out});
    }
}

#[post("/rust")]
async fn rust_request(creq: Json<CodeRequest>) -> Json<Msg> {
    if rust::write_in_file(creq.code.clone()).is_some() {
        return Json(Msg { error: true, message: "Unable to parse the Code".to_string()});
    }
    
    let out = rust::compile();
    if out.as_ref().is_some_and(|str| str.len()>1) {
        return Json(Msg { error: true, message: out.unwrap()});
    }else {
        let out = rust::execute(creq.input.clone());
        return Json(Msg { error: false, message: out});
    }
}

#[actix_web::main]
async fn main()-> Result<()> {
    HttpServer::new(||{
            App::new()
            .service(index)
            .service(c_request)
            .service(cpp_request)
            .service(rust_request)
        }
    ).
    bind(("127.0.0.0", 8000))?
    .run()
    .await
}
