use actix_files::Files;
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{self, App, HttpServer, get, post};
use askama::Template;
use clap::Parser;
use dirs_next::home_dir;
use local_ip_address::local_ip;
use qr2term::print_qr;
use std::fs;
use std::io;
use webbrowser::open;

mod cli;
use cli::Args;

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
    let template = IndexTemplate::new("crane-rs".to_string());
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
    println!("Received upload request");

    let home_directory = match home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Could not determine home directory");
            return HttpResponse::InternalServerError().body("Could not determine home directory");
        }
    };

    if let Err(e) = fs::create_dir_all(home_directory.join("Downloads/crane-rs")) {
        eprintln!("Failed to create directory: {}", e);
        return HttpResponse::InternalServerError().body("Failed to create directory");
    }

    let file_name = if let Some(file_name) = &form.file.file_name {
        file_name.clone()
    } else {
        "uploaded_file".to_string()
    };

    let file_path = format!("/home/santosh/Downloads/crane-rs/{}", file_name);

    let mut f = fs::File::create(&file_path).unwrap();
    let mut temp_file = form.file.file;

    io::copy(&mut temp_file, &mut f).unwrap();

    if let Some(file) = form.file.file_name {
        println!("File name: {:?}", file);
    }
    HttpResponse::Ok().body("Upload content")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let port = args.get_port();

    let local_ip = local_ip().unwrap();
    print_qr(&format!("http://{}:{}/", local_ip, port)).unwrap();

    if let Err(e) = open(&format!("http://{}:{}/", local_ip, port)) {
        eprintln!("Failed to open web browser: {}", e);
    }

    println!("Server running at http://{}:{}/", local_ip, port);

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(upload)
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
