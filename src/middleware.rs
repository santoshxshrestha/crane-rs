use crate::cli;
use actix_web::HttpResponse;
use actix_web::error::Error;
use actix_web::middleware::Next;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};
use clap::Parser;
use cli::Args;

pub async fn check_auth(
    req: ServiceRequest,
    next: Next<actix_web::body::BoxBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let args = Args::parse();
    let res = HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish();
    match args.get_auth() {
        Some(password) => {
            if let Some(c) = req.cookie("crane-rs") {
                if c.value() == password {
                    next.call(req).await
                } else {
                    return Ok(req.into_response(res));
                }
            } else {
                return Ok(req.into_response(res));
            }
        }
        None => next.call(req).await,
    }
}
