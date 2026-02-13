use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    response::{IntoResponse, Response},
    routing::get,
};
use gg_core::domain::prelude::User;
use sea_orm::DatabaseConnection;
use serde_json::json;

use crate::infra::{
    api::auth::{AuthLayerExt, create_auth_state},
    config::Config,
};

async fn get_current_user(Extension(user): Extension<User>) -> Response {
    Json(json!({
        "id": user.uid().value(),
        "username": user.name().value(),
    }))
    .into_response()
}

pub fn create_user_router(config: Arc<Config>, connection: DatabaseConnection) -> Router {
    Router::new()
        .route("/me", get(get_current_user))
        .with_auth(create_auth_state(config, connection))
}
