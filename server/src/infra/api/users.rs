use std::sync::Arc;

use axum::{Extension, Router, response::Response, routing::get};
use gg_core::domain::prelude::User;
use sea_orm::DatabaseConnection;
use serde_json::json;

use crate::infra::{
    api::{
        auth::{AuthLayerExt, create_auth_state},
        response::{self, ToJson},
    },
    config::Config,
};

impl ToJson for User {
    fn to_json(self) -> serde_json::Value {
        json!({
            "id": self.uid().inner(),
            "username": self.name().inner(),
        })
    }
}

async fn get_current_user(Extension(user): Extension<User>) -> Response {
    response::ok(user)
}

pub fn create_user_router(config: Arc<Config>, connection: DatabaseConnection) -> Router {
    Router::new()
        .route("/me", get(get_current_user))
        .with_auth(create_auth_state(config, connection))
}
