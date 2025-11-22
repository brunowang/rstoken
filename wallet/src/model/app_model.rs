use crate::config::server_config::Config;
use sqlx::{MySql, Pool};

pub struct AppState {
    pub db: Pool<MySql>,
    pub env: Config,
}
