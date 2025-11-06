use crate::HttpResponse;
use crate::MultipartForm;
use crate::Responder;
use crate::UploadForm;
use crate::env;
use crate::fs;
use crate::post;

#[post("/upload")]
pub async fn upload(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let start = std::time::Instant::now();
    let tmp_dir = env::temp_dir().join("crane-rs");

    if let Err(e) = fs::create_dir_all(&tmp_dir) {
        eprintln!("Failed to create directory: {e}");
        return HttpResponse::InternalServerError().body("Failed to create directory");
    }

    let file_name = form
        .file
        .file_name
        .clone()
        .unwrap_or_else(|| "uploaded_file".to_string());
    let file_path = tmp_dir.join(&file_name);

    // Move the temp file to the target location
    match form.file.file.persist(&file_path) {
        Ok(_) => {
            println!(
                "File stored: {:?}, write time: {:?}",
                file_name,
                start.elapsed()
            );
            HttpResponse::Ok()
                .content_type("text/html")
                .body(format!("File '{file_name}' uploaded successfully"))
        }
        Err(e) => {
            eprintln!("Failed to move file: {e}");
            HttpResponse::InternalServerError().body("Failed to move file")
        }
    }
}
