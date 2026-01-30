use axum::Router;
use env_logger::Env;

#[tokio::main]
async fn main() {
    // TODO: implement db connection
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app = Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();
    log::info!(target: "application", "Listening on http://127.0.0.1:3003");

    axum::serve(listener, app).await.unwrap();
}
