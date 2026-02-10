use crate::HttpResponse;
use crate::Responder;
use crate::get;
use actix_web::web;
use askama::Template;

#[derive(Template)]
#[template(path = "upload.html")]
pub struct UploadTemplate {
    css: String,
}

#[get("/upload")]
pub async fn upload_page(css: web::Data<String>) -> impl Responder {
    let template = UploadTemplate {
        css: css.as_ref().clone(),
    };

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
