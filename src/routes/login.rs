use crate::HttpResponse;
use crate::LoginTemplate;
use crate::Responder;
use crate::Template;
use crate::get;

#[get("/login")]
pub async fn login() -> impl Responder {
    let template = LoginTemplate::new();
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
