mod handlers;

use axum::Router;

use crate::{app::AppDeps, auth::public::AuthLayerExt};

pub fn mount(router: Router, deps: &AppDeps) -> Router {
    router.nest(
        "/api/v1/users",
        handlers::routes()
            .with_state(deps.clone())
            .with_auth(deps.into()),
    )
}
