mod cli;
mod config;
mod contact;
mod db;
mod errors;
mod models;
mod routes;

use config::Config;
use db::init_db;
use routes::create_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let config = Config::load();
    let pool = init_db(&config.db_path).await;

    let app = create_router(pool);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Running on {}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap()
}
