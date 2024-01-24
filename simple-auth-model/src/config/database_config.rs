use serde::Deserialize;

#[derive(Deserialize)]
pub enum DatabaseConfig {
    Sqlite(SqliteConfig)
}

#[derive(Deserialize)]
pub enum SqliteConfig {
    InMemory,
    Path(String)
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig::Sqlite(SqliteConfig::InMemory)
    }
}