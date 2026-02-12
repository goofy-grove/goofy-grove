use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::post,
};
use gg_core::{application::auth::UserAuthorizationService, domain::prelude::*};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;

use crate::infra::{
    config::Config,
    db::UserRepository,
    jwt::{JwtAccessTokenGenerator, JwtRefreshTokenGenerator},
    security::ArgonPasswordSystem,
};

#[derive(Debug, Clone)]
struct AuthorizationState<A: AuthorizationUseCase, T: TokenGeneratorPort, T1: TokenGeneratorPort> {
    user_service: A,
    access_token_generator: T,
    refresh_token_generator: T1,
}

#[derive(Debug, Clone, Deserialize)]
struct AuthorizeUserRequest {
    username: String,
    password: String,
}

async fn authorize_user<A: AuthorizationUseCase, T: TokenGeneratorPort, T1: TokenGeneratorPort>(
    State(auth_state): State<AuthorizationState<A, T, T1>>,
    Json(payload): Json<AuthorizeUserRequest>,
) -> Response {
    let command = AuthorizationCommand::new(
        UserName::new(payload.username),
        Secret::new(payload.password),
    );

    let auth_result = auth_state.user_service.authorize(command).await;

    if auth_result.is_err() {
        let mut response = Json(json!({"error": "Failed to authorize user"})).into_response();

        *(response.status_mut()) = StatusCode::UNAUTHORIZED;

        return response;
    }

    let user = auth_result.unwrap();

    let (access_token, exp) = auth_state
        .access_token_generator
        .generate_token(&user)
        .await
        .unwrap();

    let (refresh_token, refresh_token_lifetime) = auth_state
        .refresh_token_generator
        .generate_token(&user)
        .await
        .unwrap();

    let mut response = Json(json!({ "token": access_token, "exp": exp })).into_response();

    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token={};Secure;HttpOnly;Max-Age={}",
            refresh_token, refresh_token_lifetime
        ))
        .unwrap(),
    );

    response
}

pub fn create_auth_router(config: Arc<Config>, connection: DatabaseConnection) -> Router {
    let app_state = AuthorizationState {
        user_service: UserAuthorizationService::new(
            UserRepository::new(connection.clone()),
            ArgonPasswordSystem,
        ),
        access_token_generator: JwtAccessTokenGenerator::new(config.clone()),
        refresh_token_generator: JwtRefreshTokenGenerator::new(config),
    };

    Router::new()
        .route("/login", post(authorize_user))
        .with_state(app_state)
}
