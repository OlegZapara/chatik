use axum::Router;
use chatik::{check_env_vars, database};
use env_logger::Env;
use log::info;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();

    if check_env_vars() {
        log::error!("Some environment variables are missing!");
    }

    database::check_for_migrations()
        .await
        .expect("An error occurred while running migrations.");

    let pool = database::connect()
        .await
        .expect("Database connection failed");

    let app_config = chatik::app_setup(pool.clone());

    info!("Starting Axum HTTP Server");

    let app = Router::new()
        .nest("/v1", chatik::routes::v1::routes())
        .nest_service("/public", ServeDir::new("public"))
        .with_state(app_config);
    let listener = TcpListener::bind(dotenvy::var("BIND_ADDR").unwrap())
        .await
        .expect("Failed to connect");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve API");
}
