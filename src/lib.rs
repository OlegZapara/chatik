use crate::util::env::parse_var;
use axum::extract::FromRef;
use log::info;
use sqlx::PgPool;

pub mod database;
pub mod models;
pub mod routes;
pub mod util;

#[derive(Clone)]
pub struct AppConfig {
    pub pool: PgPool,
}

impl FromRef<AppConfig> for PgPool {
    fn from_ref(config: &AppConfig) -> Self {
        config.pool.clone()
    }
}

pub fn app_setup(pool: PgPool) -> AppConfig {
    info!("Starting app on {}", dotenvy::var("BIND_ADDR").unwrap());
    AppConfig { pool }
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
    failed |= check_var::<String>("DATABASE_MIN_CONNECTIONS");
    failed |= check_var::<String>("DATABASE_MAX_CONNECTIONS");
    failed
}
