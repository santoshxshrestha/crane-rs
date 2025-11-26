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

    let responses: Vec<HttpResponse> = form
        .file
        .into_iter()
        .map(|f| {
            let var = f.file.persist(
                tmp_dir.join(
                    &f.file_name
                        .clone()
                        .unwrap_or_else(|| "uploaded_file".to_string()),
                ),
            );
            match var {
                Ok(_) => {
                    println!(
                        "File stored: {:?}, write time: {:?}",
                        &f.file_name
                            .clone()
                            .unwrap_or_else(|| "uploaded_file".to_string()),
                        start.elapsed()
                    );
                    HttpResponse::Ok().content_type("text/html").body(format!(
                        "File '{}' uploaded successfully",
                        &f.file_name.unwrap_or_else(|| "uploaded_file".to_string())
                    ))
                }
                Err(e) => {
                    eprintln!("Failed to move file: {e}");
                    HttpResponse::InternalServerError().body("Failed to move file")
                }
            }
        })
        .collect();

    responses
        .into_iter()
        .next()
        .unwrap_or_else(|| HttpResponse::InternalServerError().body("No files uploaded"))
}
