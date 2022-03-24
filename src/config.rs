use config::{Config, ConfigError, File};
use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub database_url: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct HttpConfig {
    pub address: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub http: HttpConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .build()
            .unwrap();
        config.try_deserialize()
    }
}

pub(crate) fn get_config() -> &'static AppConfig {
    Setting.get_or_init(|| AppConfig::new().unwrap())
}

static Setting: OnceCell<AppConfig> = OnceCell::new();
