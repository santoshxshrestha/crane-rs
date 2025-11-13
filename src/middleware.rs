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
    match args.get_auth() {
        Some(_) => {
            let res = HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish();
            return Ok(req.into_response(res));
        }
        None => next.call(req).await,
    }
}
