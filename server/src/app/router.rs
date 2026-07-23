use axum::{Router, extract::DefaultBodyLimit};
use socketioxide::layer::SocketIoLayer;
use tower_http::cors::CorsLayer;

use crate::{app::AppDeps, auth, character, file, persona, user};

pub fn build_router(deps: &AppDeps, socketio_layer: SocketIoLayer) -> Router {
    let body_limit = deps.config.policies.max_upload_body_limit();

    let router = Router::new().layer(socketio_layer);

    let router = auth::api::mount(router, deps);
    let router = user::api::mount(router, deps);
    let router = file::api::mount(router, deps);
    let router = character::api::mount(router, deps);
    let router = persona::api::mount(router, deps);

    router
        .layer(DefaultBodyLimit::max(body_limit))
        .layer(CorsLayer::very_permissive())
}
