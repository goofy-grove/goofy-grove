use std::sync::Arc;

use env_logger::Env;

mod application;
mod infra;

use infra::config::Config;

use crate::infra::{api::server::start_server, db::init_db};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!(target: "application", "Starting server");

    log::info!(target: "application", "Loading configuration");
    let config = Arc::new(Config::from_file());

    let db_connection = init_db(&config).await;

    log::info!(target: "application", "Configuration loaded: {:?}", config);

    start_server(config, db_connection).await;
}
