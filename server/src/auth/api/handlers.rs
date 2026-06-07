use axum::{
    Json, Router,
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
    platform::http::response::{self, ToJson},
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

async fn authenticate_user(
    State(deps): State<AppDeps>,
    Json(payload): Json<AuthenticateUserRequest>,
) -> Result<Response, Response> {
    if payload.username.trim().is_empty() {
        return Err(response::bad_request(&["Invalid username"]));
    }
    if payload.password.trim().is_empty() {
        return Err(response::bad_request(&["Invalid password"]));
    }

    let user = authenticate(&deps, &payload.username, &payload.password)
        .await
        .map_err(|err| match err {
            AuthenticateError::InvalidCredentials => {
                response::auth_error(&["Failed to authenticate user"])
            }
            AuthenticateError::InternalError(_) => {
                response::internal_error(&["Failed to authenticate user"])
            }
        })?;

    let tokens = generate_tokens(&deps, &user)
        .await
        .map_err(|err| response::internal_error(&[&err.to_string()]))?;

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
) -> Result<Response, Response> {
    let refresh_token = cookie
        .get("refresh_token")
        .ok_or(response::auth_error(&["Refresh token not found"]))?
        .value();

    invalidate_device_by_token(&deps, refresh_token)
        .await
        .map_err(|_| response::auth_error(&["Token not found"]))?;

    let claims = validate_token(refresh_token, &deps.config.jwt.refresh_token)
        .map_err(|_| response::auth_error(&["Invalid token"]))?;

    let user = public::get_by_name(&deps, &claims.sub)
        .await
        .map_err(|_| response::auth_error(&["User not found"]))?;

    let tokens = generate_tokens(&deps, &user)
        .await
        .map_err(|err| response::internal_error(&[&err.to_string()]))?;

    let mut response = response::ok(TokenResponse {
        token: tokens.access_token,
        exp: tokens.access_exp,
    });

    set_refresh_cookie(&mut response, &tokens.refresh_token, tokens.refresh_max_age);

    Ok(response)
}

pub fn routes() -> Router<AppDeps> {
    Router::new()
        .route("/login", post(authenticate_user))
        .route("/refresh", post(refresh_token))
}
