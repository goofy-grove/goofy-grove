use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone, Serialize)]
pub struct Empty {}

pub fn ok<T: Serialize>(data: T) -> Response<Body> {
    (
        StatusCode::OK,
        Json(json!({"error": false, "data": data})),
    )
        .into_response()
}

pub fn created<T: Serialize>(data: T) -> Response<Body> {
    (
        StatusCode::CREATED,
        Json(json!({"error": false, "data": data})),
    )
        .into_response()
}
