use crate::HttpResponse;
use crate::IndexTemplate;
use crate::Responder;
use crate::Template;
use crate::get;

#[get("/")]
async fn index() -> impl Responder {
    let template = IndexTemplate::new("crane-rs - index".to_string());
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
