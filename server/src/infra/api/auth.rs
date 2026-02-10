use axum::{Json, Router, extract::State, routing::post};
use gg_core::{application::auth::UserAuthorizationService, domain::prelude::*};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::infra::{db::UserRepository, security::ArgonPasswordSystem};

#[derive(Debug, Clone)]
struct AuthorizationState {
    user_service: UserAuthorizationService<UserRepository, ArgonPasswordSystem>,
}

#[derive(Debug, Clone, Deserialize)]
struct AuthorizeUserRequest {
    username: String,
    password: String,
}

async fn authorize_user(
    State(auth_state): State<AuthorizationState>,
    Json(payload): Json<AuthorizeUserRequest>,
) -> String {
    let command = AuthorizationCommand::new(
        UserName::new(payload.username),
        Secret::new(payload.password),
    );

    match auth_state.user_service.authorize(command).await {
        Ok(user) => {
            log::info!(target: "application", "Authorized user: {:?}", user.name());

            format!("Authorized user: {:?}", user.name())
        }
        Err(err) => {
            log::error!(target: "application", "Failed to authorize user: {:?}", err);

            format!("Failed to authorize user: {:?}", err)
        }
    }
}

pub fn create_auth_router(connection: DatabaseConnection) -> Router {
    let app_state = AuthorizationState {
        user_service: UserAuthorizationService::new(
            UserRepository::new(connection.clone()),
            ArgonPasswordSystem,
        ),
    };

    Router::new()
        .route("/login", post(authorize_user))
        .with_state(app_state)
}
