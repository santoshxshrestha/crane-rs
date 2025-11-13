use actix_web::HttpRequest;
use actix_web::web;

use crate::HttpResponse;
use crate::Responder;
use crate::Template;
use crate::get;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

impl LoginTemplate {
    pub fn new() -> Self {
        LoginTemplate {}
    }
}

#[get("/login")]
pub async fn login(_req: HttpRequest, _auth: web::Data<Option<String>>) -> impl Responder {
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
