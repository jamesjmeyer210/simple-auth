use std::{fs, io};
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

impl Config {
    fn load(path: &str) -> Result<Config, io::Error> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(content.as_str()).unwrap_or_default())
    }
}

#[cfg(test)]
mod test {
    use crate::config::Config;
    use crate::config::database_config::{DatabaseConfig};

    #[test]
    fn config_deserializes() {
        let files = vec![
            "../test_data/model/config/001.json",
            "../test_data/model/config/002.json"
        ];

        for file in files.iter() {
            let config = Config::load(*file);
            assert!(config.is_ok())
        }
    }

    #[test]
    fn verify_config_002() {
        let config = Config::load("../test_data/model/config/002.json").unwrap();
        assert_eq!(DatabaseConfig::default(), config.database);
    }
}