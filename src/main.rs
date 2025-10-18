#![allow(unused)]
use actix_web::{self, App, HttpServer, get};
use askama::Template;
use std::io;

#[get("/")]
async fn index() -> impl actix_web::Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting server at http://127.0.0.1:8080/");
    HttpServer::new(move || App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
