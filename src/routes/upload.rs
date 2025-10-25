use crate::HttpResponse;
use crate::Responder;
use crate::Template;
use crate::UploadTemplate;
use crate::get;

#[get("/upload")]
pub async fn upload_page() -> impl Responder {
    let template = UploadTemplate::new("crane-rs - upload".to_string());
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
