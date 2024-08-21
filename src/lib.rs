use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::util::env::parse_var;
use axum::extract::{ws::Message, FromRef};
use log::info;
use redis::Client;
use sqlx::PgPool;
use tokio::sync::mpsc::UnboundedSender;

pub mod database;
pub mod models;
pub mod routes;
pub mod util;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<String, Tx>>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis_client: Client,
    pub active_connections: Arc<Mutex<u32>>,
    pub peers: PeerMap,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(config: &AppState) -> Self {
        config.pool.clone()
    }
}

pub fn app_setup(pool: PgPool, redis_client: Client) -> AppState {
    info!("Starting app on {}", dotenvy::var("BIND_ADDR").unwrap());
    AppState {
        pool,
        redis_client,
        active_connections: Arc::new(Mutex::new(0)),
        peers: Arc::new(Mutex::new(HashMap::new())),
    }
}

pub fn check_env_vars() -> bool {
    let mut failed = false;
    fn check_var<T: std::str::FromStr>(var: &'static str) -> bool {
        let check = parse_var::<T>(var).is_none();
        if check {
            log::warn!(
                "Variable `{}` missing in dotenv or not of type `{}`",
                var,
                std::any::type_name::<T>()
            );
        }
        check
    }
    failed |= check_var::<String>("BIND_ADDR");
    failed |= check_var::<String>("DATABASE_URL");
    failed |= check_var::<String>("REDIS_URL");
    failed |= check_var::<String>("DATABASE_MIN_CONNECTIONS");
    failed |= check_var::<String>("DATABASE_MAX_CONNECTIONS");
    failed |= check_var::<String>("ENCRYPTION_KEY");
    failed
}
