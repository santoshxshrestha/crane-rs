use crate::HttpResponse;
use crate::Responder;
use crate::Template;
use crate::get;
use actix_web::web;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    css: String,
}

#[get("/login")]
pub async fn login(css: web::Data<String>) -> impl Responder {
    let template = LoginTemplate {
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
