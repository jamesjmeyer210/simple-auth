use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub domain: String,
    pub port: u16,
    pub workers: Option<usize>
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            domain: String::from("127.0.0.1"),
            port: 7777,
            workers: None,
        }
    }
}