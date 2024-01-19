use chrono::{DateTime, Utc};
use serde::Serialize;

pub struct Jwt {
    pub header: Header,
    pub claims: Claims,
    pub signature: Vec<u8>
}

#[derive(Debug, Serialize)]
pub struct Header {
    pub algo: String,
    pub _type: String,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            algo: String::from("SHA256"),
            _type: String::from("JWT")
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Claims {
    pub name: String,
    pub roles: Vec<String>,
    pub realms: Vec<String>,
    pub auth_time: DateTime<Utc>,
}