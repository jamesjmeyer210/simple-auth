use serde::{Deserialize};

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub domain: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub allowed_origins: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            domain: String::from("127.0.0.1"),
            port: 7777,
            workers: None,
            allowed_origins: Vec::with_capacity(0)
        }
    }
}