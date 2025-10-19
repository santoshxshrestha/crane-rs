#![allow(unused)]
use actix_files::Files;
use actix_multipart::form::{MultipartForm, json::Json, tempfile::TempFile};
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{self, App, HttpServer, get, post};
use askama::Template;
use serde::Deserialize;
use std::fs;
use std::io::Write;

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

#[derive(MultipartForm, Debug)]
pub struct UploadForm {
    pub file: TempFile,
}

#[post("/upload")]
async fn upload(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    fs::create_dir_all("/home/santosh/Downloads/crane-rs").unwrap();

    let file_name = if let Some(file_name) = &form.file.file_name {
        file_name.clone()
    } else {
        "uploaded_file".to_string()
    };

    let file_path = format!("/home/santosh/Downloads/crane-rs/{}", file_name);
    let mut f = fs::File::create(&file_path).unwrap();

    println!("Received upload request");
    if let Some(file) = form.file.file_name {
        println!("File name: {:?}", file);
    }
    HttpResponse::Ok().body("Upload content")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting server at http://127.0.0.1:8080/");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(upload)
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
