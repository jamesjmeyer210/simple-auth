use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub domain: String,
    pub port: u16,
    pub workers: Option<u16>
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