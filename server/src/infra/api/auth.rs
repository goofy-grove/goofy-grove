use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Request, State},
    http::{HeaderValue, header},
    middleware::{self, Next},
    response::Response,
    routing::post,
};
use axum_extra::extract::CookieJar;
use gg_core::{
    application::{
        auth::UserAuthorizationService,
        tokens::{CreateDeviceService, InvalidateDeviceService},
        user::GetUserByNameService,
    },
    domain::prelude::*,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;

use crate::infra::{
    api::response::{self, ToJson},
    clock::ChronoClock,
    config::Config,
    db::{TokensRepository, UserRepository},
    id_generator::UuidGenerator,
    jwt::{
        JwtAccessTokenGenerator, JwtAccessTokenValidator, JwtRefreshTokenGenerator,
        JwtRefreshTokenValidator,
    },
    security::{ArgonPasswordSystem, ArgonTokenHasher},
};

#[derive(Debug, Clone)]
struct AuthorizationState<
    A: AuthorizationUseCase,
    T: TokenGeneratorPort,
    T1: TokenGeneratorPort,
    C: CreateDeviceUseCase,
    V: TokenValidatorPort,
    I: InvalidateDeviceUseCase,
    U: GetUserByNameQuery,
> {
    authorization_use_case: A,
    access_token_generator: T,
    refresh_token_generator: T1,
    create_device_use_case: C,
    token_validator_port: V,
    invalidate_device_use_case: I,
    get_user_by_name_query: U,
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

struct TokenResponse {
    token: String,
    exp: usize,
}

impl ToJson for TokenResponse {
    fn to_json(self) -> serde_json::Value {
        json!({ "token": self.token, "exp": self.exp })
    }
}
async fn generate_tokens(
    auth_state: AuthorizationState<
        impl AuthorizationUseCase,
        impl TokenGeneratorPort,
        impl TokenGeneratorPort,
        impl CreateDeviceUseCase,
        impl TokenValidatorPort,
        impl InvalidateDeviceUseCase,
        impl GetUserByNameQuery,
    >,
    user: &User,
) -> (Response, (String, usize)) {
    let (access_token, exp) = auth_state
        .access_token_generator
        .generate_token(user)
        .await
        .unwrap();

    let (refresh_token, refresh_token_lifetime) = auth_state
        .refresh_token_generator
        .generate_token(user)
        .await
        .unwrap();

    let _ = auth_state
        .create_device_use_case
        .create_device(CreateDeviceCommand::new(
            Token::new(refresh_token.to_owned()),
            UserAgent::new("".to_string()),
            user.uid().to_owned(),
        ))
        .await
        .unwrap();

    (
        response::ok(TokenResponse {
            token: access_token,
            exp,
        }),
        (refresh_token, refresh_token_lifetime),
    )
}

async fn refresh_token(
    State(auth_state): State<
        AuthorizationState<
            impl AuthorizationUseCase,
            impl TokenGeneratorPort,
            impl TokenGeneratorPort,
            impl CreateDeviceUseCase,
            impl TokenValidatorPort,
            impl InvalidateDeviceUseCase,
            impl GetUserByNameQuery,
        >,
    >,
    cookie: CookieJar,
) -> Result<Response, Response> {
    println!("{:?}", cookie.get("refresh_token"));
    let refresh_token = cookie
        .get("refresh_token")
        .ok_or(response::auth_error(&["Refresh token not found"]))?
        .value();
    let token = Token::new(refresh_token.to_owned());

    auth_state
        .invalidate_device_use_case
        .invalidate_device(InvalidateDeviceCommand::new(Token::new(
            refresh_token.to_owned(),
        )))
        .await
        .or(Err(response::auth_error(&["Token not found"])))?;

    let token_data = auth_state
        .token_validator_port
        .validate_token(&token)
        .await
        .or(Err(response::auth_error(&["Invalid token"])))?;

    let user = auth_state
        .get_user_by_name_query
        .get_user_by_name(&UserName::new(token_data.username().to_owned()))
        .await
        .or(Err(response::auth_error(&["User not found"])))?;

    let (mut response, (refresh_token, refresh_token_lifetime)) =
        generate_tokens(auth_state, &user).await;

    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token={};Secure;HttpOnly;SameSite=None;Max-Age={};Path=/api/v1/auth/refresh",
            refresh_token, refresh_token_lifetime
        ))
        .unwrap(),
    );

    Ok(response)
}

async fn authorize_user(
    State(auth_state): State<
        AuthorizationState<
            impl AuthorizationUseCase,
            impl TokenGeneratorPort,
            impl TokenGeneratorPort,
            impl CreateDeviceUseCase,
            impl TokenValidatorPort,
            impl InvalidateDeviceUseCase,
            impl GetUserByNameQuery,
        >,
    >,
    Json(payload): Json<AuthorizeUserRequest>,
) -> Response {
    let command = AuthorizationCommand::new(
        UserName::new(payload.username),
        Secret::new(payload.password),
    );

    let auth_result = auth_state.authorization_use_case.authorize(command).await;

    if auth_result.is_err() {
        return response::auth_error(&["Failed to authorize user"]);
    }

    let user = auth_result.unwrap();

    let (mut response, (refresh_token, refresh_token_lifetime)) =
        generate_tokens(auth_state, &user).await;

    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token={};Secure;HttpOnly;SameSite=None;Max-Age={};Path=/api/v1/auth/refresh",
            refresh_token, refresh_token_lifetime
        ))
        .unwrap(),
    );

    response
}

async fn authentication_layer(
    State(auth_state): State<AuthenticationState<impl TokenValidatorPort, impl LoadUserByNamePort>>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(response::auth_error(&["Token not found"]));
    };

    if let Some(current_user) = autheticate_current_user(auth_header, auth_state).await {
        req.extensions_mut().insert(current_user);

        Ok(next.run(req).await)
    } else {
        Err(response::auth_error(&["Failed to authenticate user"]))
    }
}

async fn autheticate_current_user(
    auth_header: &str,
    auth_state: AuthenticationState<impl TokenValidatorPort, impl LoadUserByNamePort>,
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
        create_device_use_case: CreateDeviceService::new(
            TokensRepository::new(connection.clone()),
            ArgonTokenHasher,
            UuidGenerator,
            ChronoClock,
        ),
        token_validator_port: JwtRefreshTokenValidator::new(config.clone()),
        invalidate_device_use_case: InvalidateDeviceService::new(
            TokensRepository::new(connection.clone()),
            ArgonTokenHasher,
        ),
        get_user_by_name_query: GetUserByNameService::new(UserRepository::new(connection.clone())),
    };

    Router::new()
        .route("/login", post(authorize_user))
        .route("/refresh", post(refresh_token))
        .with_state(app_state)
}
