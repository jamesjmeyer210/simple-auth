mod config;
mod server_config;
mod database_config;

pub type Config = config::Config;
pub type DatabaseConfig = database_config::DatabaseConfig;
pub type SqliteConfig = database_config::SqliteConfig;