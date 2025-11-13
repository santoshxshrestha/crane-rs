use crate::HttpResponse;
use crate::Responder;
use crate::actix_web::web::Form;
use crate::post;
use crate::web;
use actix_web::cookie::Cookie;
use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
pub struct UserPass {
    pub password: String,
}

#[post("/authentication")]
pub async fn authentication(
    auth: web::Data<Option<String>>,
    form: Form<UserPass>,
) -> impl Responder {
    if let Some(password) = auth.get_ref() {
        if form.password == *password {
            HttpResponse::Found()
                .append_header(("Location", "/"))
                .cookie(Cookie::new("crane-rs", form.password.clone()))
                .finish()
        } else {
            HttpResponse::Unauthorized().body("Invalid password")
        }
    } else {
        HttpResponse::Ok().body("No authentication required")
    }
}
