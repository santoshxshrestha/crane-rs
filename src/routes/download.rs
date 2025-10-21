use crate::DownloadTemplate;
use crate::HttpResponse;
use crate::Responder;
use crate::Template;
use crate::WalkDir;
use crate::get;
use std::env;

#[get("/download")]
async fn download_page() -> impl Responder {
    let tmp_dir = env::temp_dir().join("crane-rs");

    if !tmp_dir.exists() {
        return HttpResponse::NotFound().body("No files available for download");
    }

    let mut files = Vec::new();

    for entry in WalkDir::new(&tmp_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading directory entry: {e}");
                continue;
            }
        };

        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf())
        }
    }

    let template = DownloadTemplate::new(files, "crane-rs - download".to_string());
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}
