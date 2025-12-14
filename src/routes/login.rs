use crate::HttpResponse;
use crate::Responder;
use crate::Template;
use crate::get;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

#[get("/login")]
pub async fn login() -> impl Responder {
    let template = LoginTemplate;
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
