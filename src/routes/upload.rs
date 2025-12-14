use crate::HttpResponse;
use crate::Responder;
use crate::get;
use askama::Template;

#[derive(Template)]
#[template(path = "upload.html")]
pub struct UploadTemplate;

#[get("/upload")]
pub async fn upload_page() -> impl Responder {
    let template = UploadTemplate;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(match template.render() {
            Ok(rendered) => rendered,
            Err(e) => {
                eprintln!("Error rendering template: {e}");
                return HttpResponse::InternalServerError().body("Failed to render template");
            }
        })
}
