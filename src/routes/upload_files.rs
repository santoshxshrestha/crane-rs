use crate::HttpResponse;
use crate::MultipartForm;
use crate::Responder;
use crate::UploadForm;
use crate::env;
use crate::fs;
use crate::io;
use crate::post;

#[post("/upload")]
pub async fn upload(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
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
