extern crate base64;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use base64::{decode, encode};
use reqwest;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::stdin;
use std::str;
use std::time::Duration;




#[get("/")]
async fn hello() -> impl Responder {
    println!("Name,Certificate,Date:");
    let mut name_string = String::new();
    stdin()
        .read_line(&mut name_string)
        .ok()
        .expect("Failed to read line");

    let data = encode(name_string).to_string();
    println!("Your encoded data is: {}", data);
    HttpResponse::Ok().body(data)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}