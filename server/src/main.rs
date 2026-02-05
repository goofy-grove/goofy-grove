use axum::Router;
use env_logger::Env;

mod infra;
mod application;

use infra::config::Config;

#[tokio::main]
async fn main() {
    // TODO: implement db connection
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::from_file();

    log::info!(target: "application", "Configuration loaded: {:?}", config);

    let app = Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind(config.to_socket_addr())
        .await
        .unwrap();

    log::info!(target: "application", "Listening on {}:{}", config.host, config.port);

    axum::serve(listener, app).await.unwrap();
}
