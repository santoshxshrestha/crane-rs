#![allow(unused)]
use actix_files::Files;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{self, App, HttpServer, get};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    content: String,
}

impl IndexTemplate {
    fn new(content: String) -> Self {
        IndexTemplate { content }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    let template = IndexTemplate::new("Hello, Askama with Actix-web!".to_string());
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting server at http://127.0.0.1:8080/");
    HttpServer::new(move || App::new()
        .service(index)
        .service(Files::new("/static", "./static").show_files_listing()))
        .bind("127.0.0.1:8080")?
        .run()
    .await
}
