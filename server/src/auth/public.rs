use std::sync::Arc;

use axum::{
    Router,
    extract::{Request, State},
    http::header,
    middleware::{self, Next},
    response::Response,
};
use sea_orm::DatabaseConnection;

use crate::{
    app::AppDeps,
    auth::services::jwt::validate_token,
    platform::{config::Config, http::response},
    user::public,
};

pub use crate::user::public::User as AuthenticatedUser;

#[derive(Debug, Clone)]
pub struct AuthMiddlewareState {
    pub config: Arc<Config>,
    pub db: DatabaseConnection,
}

impl From<&AppDeps> for AuthMiddlewareState {
    fn from(deps: &AppDeps) -> Self {
        Self {
            config: deps.config.clone(),
            db: deps.db.clone(),
        }
    }
}

pub async fn resolve_user_with_token_expiry(
    state: &AuthMiddlewareState,
    token: &str,
) -> Option<(AuthenticatedUser, usize)> {
    let claims = validate_token(token, &state.config.jwt.access_token).ok()?;
    let user = public::get_by_name_db(&state.db, &claims.sub).await.ok()?;

    Some((user, claims.exp))
}

pub trait AuthLayerExt {
    fn with_auth(self, state: AuthMiddlewareState) -> Self;
}

impl<S> AuthLayerExt for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_auth(self, state: AuthMiddlewareState) -> Self {
        self.layer(middleware::from_fn_with_state(
            state,
            authentication_middleware,
        ))
    }
}

async fn authentication_middleware(
    State(state): State<AuthMiddlewareState>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = auth_header.ok_or_else(|| response::auth_error(&["Token not found"]))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| response::auth_error(&["Failed to authenticate user"]))?;

    let user = resolve_user_with_token_expiry(&state, token)
        .await
        .map(|(user, _)| user)
        .ok_or_else(|| response::auth_error(&["Failed to authenticate user"]))?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
