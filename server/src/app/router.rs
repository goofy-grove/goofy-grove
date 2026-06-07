use axum::Router;
use socketioxide::layer::SocketIoLayer;
use tower_http::cors::CorsLayer;

use crate::{app::AppDeps, auth, character, file, persona, user};

pub fn build_router(deps: &AppDeps, socketio_layer: SocketIoLayer) -> Router {
    let router = Router::new().layer(socketio_layer);

    let router = auth::api::mount(router, deps);
    let router = user::api::mount(router, deps);
    let router = file::api::mount(router, deps);
    let router = character::api::mount(router, deps);
    let router = persona::api::mount(router, deps);

    router.layer(CorsLayer::very_permissive())
}
