use crate::HttpResponse;
use crate::Responder;
use crate::Template;
use crate::get;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[get("/")]
async fn index() -> impl Responder {
    let template = IndexTemplate;
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
