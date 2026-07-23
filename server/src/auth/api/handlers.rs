use axum::{
    Router,
    extract::State,
    http::{HeaderValue, header},
    response::Response,
    routing::post,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use serde_json::json;

use crate::{
    app::AppDeps,
    auth::services::{
        authenticate::{AuthenticateError, authenticate},
        device::invalidate_device_by_token,
        jwt::validate_token,
        tokens::generate_tokens,
    },
    platform::http::{
        error::{ApiError, codes},
        extract::ValidatedJson,
        response::{self, ToJson},
    },
    user::public,
};

#[derive(Debug, Clone, Deserialize)]
struct AuthenticateUserRequest {
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

fn set_refresh_cookie(response: &mut Response, refresh_token: &str, max_age: usize) {
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token={};Secure;HttpOnly;SameSite=None;Max-Age={};Path=/api/v1/auth/refresh",
            refresh_token, max_age
        ))
        .unwrap(),
    );
}

fn clear_refresh_cookie(response: &mut Response) {
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(
            "refresh_token=;Secure;HttpOnly;SameSite=None;Max-Age=0;Path=/api/v1/auth/refresh",
        )
        .unwrap(),
    );
}

async fn authenticate_user(
    State(deps): State<AppDeps>,
    ValidatedJson(payload): ValidatedJson<AuthenticateUserRequest>,
) -> Result<Response, ApiError> {
    if payload.username.trim().is_empty() {
        return Err(ApiError::bad_request(codes::AUTH_INVALID_USERNAME));
    }
    if payload.password.trim().is_empty() {
        return Err(ApiError::bad_request(codes::AUTH_INVALID_PASSWORD));
    }

    let user = authenticate(&deps, &payload.username, &payload.password)
        .await
        .map_err(|err| match err {
            AuthenticateError::InvalidCredentials => {
                ApiError::unauthorized(codes::AUTH_INVALID_CREDENTIALS)
            }
            AuthenticateError::InternalError(_) => {
                ApiError::internal(codes::AUTH_AUTHENTICATION_FAILED)
            }
        })?;

    let tokens = generate_tokens(&deps, &user)
        .await
        .map_err(|_| ApiError::internal(codes::AUTH_TOKEN_GENERATION_FAILED))?;

    let mut response = response::ok(TokenResponse {
        token: tokens.access_token,
        exp: tokens.access_exp,
    });

    set_refresh_cookie(&mut response, &tokens.refresh_token, tokens.refresh_max_age);

    Ok(response)
}

async fn refresh_token(
    State(deps): State<AppDeps>,
    cookie: CookieJar,
) -> Result<Response, ApiError> {
    let refresh_token = cookie
        .get("refresh_token")
        .ok_or(ApiError::unauthorized(codes::AUTH_REFRESH_TOKEN_NOT_FOUND))?
        .value();

    invalidate_device_by_token(&deps, refresh_token)
        .await
        .map_err(|_| ApiError::unauthorized(codes::AUTH_TOKEN_NOT_FOUND))?;

    let claims = validate_token(refresh_token, &deps.config.jwt.refresh_token)
        .map_err(|_| ApiError::unauthorized(codes::AUTH_TOKEN_INVALID))?;

    let user = public::get_by_name(&deps, &claims.sub)
        .await
        .map_err(|_| ApiError::unauthorized(codes::AUTH_USER_NOT_FOUND))?;

    let tokens = generate_tokens(&deps, &user)
        .await
        .map_err(|_| ApiError::internal(codes::AUTH_TOKEN_GENERATION_FAILED))?;

    let mut response = response::ok(TokenResponse {
        token: tokens.access_token,
        exp: tokens.access_exp,
    });

    set_refresh_cookie(&mut response, &tokens.refresh_token, tokens.refresh_max_age);

    Ok(response)
}

#[derive(Debug, Clone)]
struct LogoutResponse;

impl ToJson for LogoutResponse {
    fn to_json(self) -> serde_json::Value {
        json!({})
    }
}

async fn logout_user(State(deps): State<AppDeps>, cookie: CookieJar) -> Result<Response, ApiError> {
    if let Some(refresh_token) = cookie.get("refresh_token") {
        let _ = invalidate_device_by_token(&deps, refresh_token.value()).await;
    }

    let mut response = response::ok(LogoutResponse);
    clear_refresh_cookie(&mut response);

    Ok(response)
}

pub fn routes() -> Router<AppDeps> {
    Router::new()
        .route("/login", post(authenticate_user))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout_user))
}
