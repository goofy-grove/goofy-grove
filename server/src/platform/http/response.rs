use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

pub trait ToJson {
    fn to_json(self) -> serde_json::Value;
}

impl<T: ToJson> ToJson for Vec<T> {
    fn to_json(self) -> serde_json::Value {
        self.into_iter().map(|item| item.to_json()).collect()
    }
}

pub fn ok<T: ToJson>(data: T) -> Response<Body> {
    (
        StatusCode::OK,
        Json(json!({"error": false, "data": data.to_json()})),
    )
        .into_response()
}

pub fn created<T: ToJson>(data: T) -> Response<Body> {
    (
        StatusCode::CREATED,
        Json(json!({"error": false, "data": data.to_json()})),
    )
        .into_response()
}
