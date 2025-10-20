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
use cli::Args;

#[derive(Template)]
#[template(path = "upload.html")]
struct UploadTemplate {
    content: String,
}

impl UploadTemplate {
    fn new(content: String) -> Self {
        Self { content }
    }
}

#[get("/upload")]
async fn upload_page() -> impl Responder {
    let template = UploadTemplate::new("crane-rs - upload".to_string());
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "download.html")]
struct DownloadTemplate {
    content: String,
    files: Vec<FileInfo>,
}

struct FileInfo {
    name: String,
    file: String,
}
impl FileInfo {
    fn new(name: String, file: String) -> Self {
        Self { name, file }
    }
}

impl DownloadTemplate {
    fn new(files: Vec<PathBuf>, content: String) -> Self {
        DownloadTemplate {
            files: files
                .into_iter()
                .map(|path| {
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let file = path.to_string_lossy().to_string();
                    FileInfo::new(name, file)
                })
                .collect(),
            content,
        }
    }
}

#[get("/download")]
async fn download_page() -> impl Responder {
    let tmp_dir = env::temp_dir().join("crane-rs");

    if !tmp_dir.exists() {
        return HttpResponse::NotFound().body("No files available for download");
    }

    let mut files = Vec::new();

    for entry in WalkDir::new(&tmp_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf())
        }
    }

    let template = DownloadTemplate::new(files, "crane-rs - download".to_string());
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

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
    let template = IndexTemplate::new("crane-rs - index".to_string());
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
    let tmp_dir = env::temp_dir();

    if let Err(e) = fs::create_dir_all(tmp_dir.join("crane-rs")) {
        eprintln!("Failed to create directory: {}", e);
        return HttpResponse::InternalServerError().body("Failed to create directory");
    }

    let file_name = if let Some(file_name) = &form.file.file_name {
        file_name.clone()
    } else {
        "uploaded_file".to_string()
    };

    let file_path = tmp_dir.join("crane-rs").join(&file_name);

    let mut f = fs::File::create(&file_path).unwrap();
    let mut temp_file = form.file.file;

    io::copy(&mut temp_file, &mut f).unwrap();

    if let Some(file) = form.file.file_name {
        println!("File name: {:?}", file);
    }
    HttpResponse::Ok().body("Upload content")
}

fn copy_files_to_temp(files: Vec<PathBuf>) -> std::io::Result<()> {
    let tmp_dir = env::temp_dir().join("crane-rs");

    if let Err(e) = fs::create_dir_all(&tmp_dir) {
        eprintln!("Failed to create directory: {}", e);
        return Err(e);
    };

    for file in files {
        let file_name = file
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let dest_path = tmp_dir.join(&file_name);
        fs::copy(&file, &dest_path)?;
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let port = args.get_port();
    let files = args.get_files();

    if !files.is_empty() {
        if let Err(e) = copy_files_to_temp(files.clone()) {
            eprintln!("Failed to copy files to temp directory: {}", e);
            return Err(e);
        }
    }

    let local_ip = local_ip().unwrap();
    print_qr(&format!("http://{}:{}/", local_ip, port)).unwrap();

    if let Err(e) = open(&format!("http://{}:{}/", local_ip, port)) {
        eprintln!("Failed to open web browser: {}", e);
    }

    println!("Server running at http://{}:{}/", local_ip, port);

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(upload_page)
            .service(download_page)
            .service(upload)
            .service(Files::new("/static", "./static").show_files_listing())
            .service(Files::new("/tmp/crane-rs", "/tmp/crane-rs").show_files_listing())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
