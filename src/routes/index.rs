use crate::HttpResponse;
use crate::Responder;
use crate::Template;
use crate::get;
use actix_web::web;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    css: String,
}

#[get("/")]
async fn index(css: web::Data<String>) -> impl Responder {
    let template = IndexTemplate {
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
