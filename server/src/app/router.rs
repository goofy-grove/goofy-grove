use axum::{Router, extract::DefaultBodyLimit};
use socketioxide::layer::SocketIoLayer;
use tower_http::cors::CorsLayer;

use crate::{app::AppDeps, auth, character, chat, file, messages, persona, user};

pub fn build_router(deps: &AppDeps, socketio_layer: SocketIoLayer) -> Router {
    let body_limit = deps.config.policies.max_upload_body_limit();

    let router = Router::new().layer(socketio_layer);

    let router = auth::mount(router, deps);
    let router = user::mount(router, deps);
    let router = file::mount(router, deps);
    let router = character::mount(router, deps);
    let router = persona::mount(router, deps);
    let router = chat::mount(router, deps);
    let router = messages::mount(router, deps);

    router
        .layer(DefaultBodyLimit::max(body_limit))
        .layer(CorsLayer::very_permissive())
}
