use chrono::{DateTime, Utc};
use serde::Serialize;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use crate::abs::AsJson;

pub struct Jwt {
    pub header: JwtHeader,
    pub claims: JwtClaims,
    pub signature: Vec<u8>
}

impl Jwt {
    pub fn to_base64_string(self) -> String {
        let h = self.header.as_json().unwrap();
        let c = self.claims.as_json().unwrap();
        format!("{}.{}.{}",
            URL_SAFE_NO_PAD.encode(&h),
            URL_SAFE_NO_PAD.encode(&c),
            URL_SAFE_NO_PAD.encode(self.signature.as_slice()))
    }
}

#[derive(Debug, Serialize)]
pub struct JwtHeader {
    pub algo: String,
    pub _type: String,
}

impl Default for JwtHeader {
    fn default() -> Self {
        Self {
            algo: String::from("SHA256"),
            _type: String::from("JWT")
        }
    }
}

#[derive(Debug, Serialize)]
pub struct JwtClaims {
    pub name: String,
    pub roles: Vec<String>,
    pub realms: Vec<String>,
    pub auth_time: DateTime<Utc>,
}