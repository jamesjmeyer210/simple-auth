use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum DatabaseConfig {
    Sqlite(SqliteConfig)
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig::Sqlite(SqliteConfig::InMemory)
    }
}

impl PartialEq for DatabaseConfig {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Sqlite(s) => match other {
                DatabaseConfig::Sqlite(o) => s == o
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum SqliteConfig {
    InMemory,
    Path(String)
}

impl PartialEq for SqliteConfig {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::InMemory => match other {
                SqliteConfig::InMemory => true,
                _ => false
            }
            Self::Path(s_path) => match other {
                SqliteConfig::Path(o_path) => s_path == o_path,
                _ => false
            }
        }
    }
}