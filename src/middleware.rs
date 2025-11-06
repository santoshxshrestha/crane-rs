use actix_web::error::Error;
use actix_web::middleware::Next;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};

pub async fn check_auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("hit the middleware");
    next.call(req).await
}
