use std::sync::Arc;

use env_logger::Env;

mod infra;

use infra::config::Config;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::infra::{api::server::start_server, db::init_db};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!(target: "application", "Starting server");
    info!(target: "application", "Loading configuration");

    let config = Arc::new(Config::from_file());

    let db_connection = init_db(&config).await;

    info!(target: "application", "Configuration loaded");

    start_server(config, db_connection).await
}
