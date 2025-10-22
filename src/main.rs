use actix_files::Files;
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{self, App, HttpServer, get, post};
use askama::Template;
use clap::Parser;
use local_ip_address::local_ip;
use qr2term::print_qr;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;
use webbrowser::open;

mod cli;
mod routes;
mod templates;
mod utils;
use cli::Args;
use routes::download::download_page;
use routes::index::index;
use routes::upload::upload_page;
use routes::upload_files::upload;
use templates::download::DownloadTemplate;
use templates::index::IndexTemplate;
use templates::upload::UploadTemplate;
use utils::store::copy_files_to_temp;
use utils::types::FileInfo;
use utils::types::UploadForm;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let temp_dir = env::temp_dir().join("crane-rs");
    let args = Args::parse();

    let port = args.get_port();
    let files = args.get_files();
    let nuke = args.get_nuke();

    if nuke && temp_dir.exists() {
        fs::remove_dir_all(temp_dir)?;
        println!("Temporary directory nuked.");
    }

    if !files.is_empty() {
        if let Err(e) = copy_files_to_temp(files.clone()) {
            eprintln!("Failed to copy files to temp directory: {e}");
            return Err(e);
        }
    }

    let local_ip = match local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Failed to get local IP address: {e}");
            return Err(io::Error::other("Failed to get local IP"));
        }
    };

    if let Err(e) = print_qr(format!("http://{local_ip}:{port}/")) {
        eprintln!("Failed to generate QR code: {e}");
    }

    if let Err(e) = open(&format!("http://{local_ip}:{port}/")) {
        eprintln!("Failed to open web browser: {e}");
    }

    println!("Server running at http://{local_ip}:{port}/");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(upload_page)
            .service(download_page)
            .service(upload)
            .service(Files::new("/tmp/crane-rs", "/tmp/crane-rs").show_files_listing())
    })
    .bind(format!("0.0.0.0:{port}"))?
    .run()
    .await
}
