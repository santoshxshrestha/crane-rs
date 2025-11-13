use actix_web::error::Error;
use actix_web::middleware::Next;
use actix_web::{HttpResponse, web};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};

pub async fn check_auth(
    req: ServiceRequest,
    next: Next<actix_web::body::BoxBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if req.path() == "/login" {
        return next.call(req).await;
    }

    if req.path() == "/authentication" {
        return next.call(req).await;
    }

    let res = HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish();

    let auth = req.app_data::<web::Data<Option<String>>>().unwrap();
    if let Some(password) = auth.get_ref() {
        if let Some(c) = req.cookie("crane-rs") {
            if c.value() == password {
                next.call(req).await
            } else {
                Ok(req.into_response(res))
            }
        } else {
            Ok(req.into_response(res))
        }
    } else {
        next.call(req).await
    }
}
