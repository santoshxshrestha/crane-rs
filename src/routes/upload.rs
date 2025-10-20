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
        .body(template.render().unwrap())
}
