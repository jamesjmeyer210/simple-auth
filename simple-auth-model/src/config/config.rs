use serde::Deserialize;
use crate::config::database_config::DatabaseConfig;
use crate::config::server_config::ServerConfig;

/// The global configuration for the application
#[derive(Deserialize)]
pub struct Config {
    /// The `actix-web` server configuration
    pub server: ServerConfig,
    /// The `database` configuration
    pub database: DatabaseConfig,
    /// The path of the logging configuration
    pub log_file: String,
    /// Indicates that the [`Config`] should be printed when true
    pub print: bool,
    /// The path to an optional banner file which will print on startup
    pub banner: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            log_file: String::from("logcfg"),
            print: false,
            banner: None,
        }
    }
}