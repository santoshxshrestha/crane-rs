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
pub async fn login(req: HttpRequest, auth: web::Data<Option<String>>) -> impl Responder {
    let _template = LoginTemplate::new();
    if let Some(password) = auth.get_ref() {
        if let Some(c) = req.cookie("crane-rs") {
            if c.value() == *password {
                return HttpResponse::Ok().body("Already authenticated from cookie");
            } else {
                return HttpResponse::Ok().body("here loign page was supposed to be rendered");
            }
        } else {
            return HttpResponse::Ok().body("here login page was supposed to be rendered");
        }
    } else {
        return HttpResponse::Ok().body("here login page was supposed to be rendered");
    }
}
