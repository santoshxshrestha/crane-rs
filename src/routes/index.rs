use crate::HttpResponse;
use crate::Responder;
use crate::Template;
use crate::get;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    content: String,
}
impl IndexTemplate {
    pub fn new(content: String) -> Self {
        IndexTemplate { content }
    }
}

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
