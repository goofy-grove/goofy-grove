mod handlers;

use axum::Router;

use crate::app::AppDeps;

pub fn mount(router: Router, deps: &AppDeps) -> Router {
    router.nest("/api/v1/auth", handlers::routes().with_state(deps.clone()))
}
