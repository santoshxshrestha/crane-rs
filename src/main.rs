use actix_files::Files;
use actix_multipart::form::MultipartFormConfig;
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::Responder;
use actix_web::middleware::from_fn;
use actix_web::{self, App, HttpServer, get, post};
use actix_web::{HttpResponse, web};
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
use cli::Args;
use routes::authentication::authentication;
use routes::download::download_page;
use routes::index::index;
use routes::login::login;
use routes::upload::upload_page;
use routes::upload_files::upload;
mod store;
use store::copy_files_to_temp;
mod types;
use types::FileInfo;
use types::UploadForm;
mod middleware;
use middleware::check_auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let temp_dir = env::temp_dir().join("crane-rs");
    let args = Args::parse();

    let port = args.get_port();
    let files = args.get_files();
    let nuke = args.get_nuke();
    let auth = args.get_auth();

    if nuke && temp_dir.exists() {
        fs::remove_dir_all(temp_dir)?;
        println!("Temporary directory nuked.");
    }

    if !files.is_empty()
        && let Err(e) = copy_files_to_temp(files.clone())
    {
        eprintln!("Failed to copy files to temp directory: {e}");
        return Err(e);
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
            .service(login)
            .service(authentication)
            .wrap(from_fn(check_auth))
            .app_data(web::Data::new(auth.clone()))
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(10 * 1024 * 1024 * 1024) // 10 GB
                    .memory_limit(10 * 1024 * 1024), // 10 MB
            )
            .service(Files::new("/tmp/crane-rs", "/tmp/crane-rs").show_files_listing())
    })
    .bind(format!("0.0.0.0:{port}"))?
    .run()
    .await
}
