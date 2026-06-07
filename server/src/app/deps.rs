use std::sync::Arc;

use sea_orm::DatabaseConnection;
use socketioxide::SocketIo;

use crate::platform::{config::Config, events::InMemoryEventBus};

#[derive(Clone)]
pub struct AppDeps {
    pub config: Arc<Config>,
    pub db: DatabaseConnection,
    pub event_bus: InMemoryEventBus,
    pub socket: SocketIo,
}
