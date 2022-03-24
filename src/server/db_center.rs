use std::time::Duration;

use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::get_config;

pub async fn init_db() {
    let config = get_config();
    let mut op = ConnectOptions::new(config.database.database_url.clone());
    op.min_connections(config.database.min_connections)
        .max_connections(config.database.max_connections)
        .idle_timeout(Duration::from_secs(config.database.idle_timeout_seconds))
        .connect_timeout(Duration::from_secs(config.database.connect_timeout_seconds))
        .sqlx_logging(true);
    DB.set(Database::connect(op).await.unwrap()).unwrap();
}

pub(crate) fn get_db() -> &'static DatabaseConnection {
    DB.get().unwrap()
}

static DB: OnceCell<DatabaseConnection> = OnceCell::new();
