use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Request, State},
    http::{HeaderValue, StatusCode, header},
    middleware::{self, Next},
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
    jwt::{JwtAccessTokenGenerator, JwtAccessTokenValidator, JwtRefreshTokenGenerator},
    security::ArgonPasswordSystem,
};

#[derive(Debug, Clone)]
struct AuthorizationState<A: AuthorizationUseCase, T: TokenGeneratorPort, T1: TokenGeneratorPort> {
    authorization_use_case: A,
    access_token_generator: T,
    refresh_token_generator: T1,
}

#[derive(Debug, Clone)]
pub struct AuthenticationState<V: TokenValidatorPort, L: LoadUserByNamePort> {
    access_token_validator: V,
    load_user_use_case: L,
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

    let auth_result = auth_state.authorization_use_case.authorize(command).await;

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

async fn authentication_layer<V: TokenValidatorPort, L: LoadUserByNamePort>(
    State(auth_state): State<AuthenticationState<V, L>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = autheticate_current_user(auth_header, auth_state).await {
        req.extensions_mut().insert(current_user);

        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn autheticate_current_user<V: TokenValidatorPort, L: LoadUserByNamePort>(
    auth_header: &str,
    auth_state: AuthenticationState<V, L>,
) -> Option<User> {
    if let Some(token) = auth_header.strip_prefix("Bearer ") {
        let token_data = auth_state
            .access_token_validator
            .validate_token(&Token::new(token.to_string()))
            .await
            .ok()?;

        let user = auth_state
            .load_user_use_case
            .load_user_by_name(&UserName::new(token_data.username().to_owned()))
            .await
            .ok()?;

        return Some(user);
    }

    None
}

pub fn create_auth_state(
    config: Arc<Config>,
    connection: DatabaseConnection,
) -> AuthenticationState<JwtAccessTokenValidator, UserRepository> {
    AuthenticationState {
        access_token_validator: JwtAccessTokenValidator::new(config),
        load_user_use_case: UserRepository::new(connection),
    }
}

pub trait AuthLayerExt {
    fn with_auth(self, state: AuthenticationState<JwtAccessTokenValidator, UserRepository>)
    -> Self;
}

impl<S> AuthLayerExt for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_auth(
        self,
        state: AuthenticationState<JwtAccessTokenValidator, UserRepository>,
    ) -> Self {
        self.layer(middleware::from_fn_with_state(state, authentication_layer))
    }
}

pub fn create_auth_router(config: Arc<Config>, connection: DatabaseConnection) -> Router {
    let app_state = AuthorizationState {
        authorization_use_case: UserAuthorizationService::new(
            UserRepository::new(connection.clone()),
            ArgonPasswordSystem,
        ),
        access_token_generator: JwtAccessTokenGenerator::new(config.clone()),
        refresh_token_generator: JwtRefreshTokenGenerator::new(config.clone()),
    };

    Router::new()
        .route("/login", post(authorize_user))
        .with_state(app_state)
}
