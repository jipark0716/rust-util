use crate::State;
use actix_web::cookie::time::OffsetDateTime;
use actix_web::web;
use clickhouse::Row;
use entity::Entity;
use rest::response::NoContentResponse;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use util_proc::Entity;
use uuid::Uuid;

#[instrument(skip(state, req, body))]
pub async fn capture(
    state: web::Data<State>,
    req: actix_web::HttpRequest,
    body: web::Bytes,
) -> NoContentResponse {
    let entity = HttpRequestEntity::from_req(req, body);

    if let Err(e) = state.http_request_buffer.send(entity.clone()).await {
        tracing::error!("fail create entity: {:?} e: {}", entity, e);
    }

    NoContentResponse {}
}

#[derive(Entity, Debug, Clone, Serialize, Deserialize, Row)]
#[entity(table = "http_request")]
pub struct HttpRequestEntity {
    #[serde(with = "clickhouse::serde::uuid")]
    pub uuid: Uuid,
    pub path: String,
    pub method: String,
    pub query_name: Vec<String>,
    pub query_value: Vec<String>,
    pub header_name: Vec<String>,
    pub header_value: Vec<String>,
    pub body: Option<String>,
    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    pub timestamp: OffsetDateTime,
}

impl HttpRequestEntity {
    pub fn from_req(req: actix_web::HttpRequest, body: web::Bytes) -> Self {
        let uuid = Uuid::now_v7();
        let path = req.path().to_string();
        let method = req.method().as_str().to_string();

        let mut query_name = Vec::new();
        let mut query_value = Vec::new();
        for (k, v) in url::form_urlencoded::parse(req.query_string().as_bytes()) {
            query_name.push(k.into_owned());
            query_value.push(v.into_owned());
        }

        let mut header_name = Vec::new();
        let mut header_value = Vec::new();
        for (k, v) in req.headers().iter() {
            header_name.push(k.as_str().to_string());
            header_value.push(v.to_str().unwrap_or_default().to_string());
        }

        let body = if body.is_empty() {
            None
        } else {
            Some(String::from_utf8_lossy(&body).to_string())
        };

        Self {
            uuid,
            path,
            method,
            query_name,
            query_value,
            header_name,
            header_value,
            body,
            timestamp: OffsetDateTime::now_utc(),
        }
    }
}
