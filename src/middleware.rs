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
    println!("Checking auth for path: {}", req.path());
    // Don't check auth for /login path
    if req.path() == "/login" {
        return next.call(req).await;
    }

    if req.path() == "/authentication" {
        return next.call(req).await;
    }

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
                    Ok(req.into_response(res))
                }
            } else {
                Ok(req.into_response(res))
            }
        }
        None => next.call(req).await,
    }
}
