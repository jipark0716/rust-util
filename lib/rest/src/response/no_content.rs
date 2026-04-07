use actix_web::body::BoxBody;
use actix_web::{HttpResponse, Responder};
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct NoContentResponse();

impl Responder for NoContentResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        HttpResponse::build(StatusCode::NO_CONTENT).finish()
    }
}
