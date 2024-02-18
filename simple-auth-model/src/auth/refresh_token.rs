use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

#[derive(Debug, Serialize)]
pub struct RefreshToken {
    pub user_id: Uuid,
    pub issued_on: DateTime<Utc>,
}

impl RefreshToken {
    pub fn new(user_id: &Uuid, issued_on: &DateTime<Utc>) -> RefreshToken {
        Self {
            user_id: *user_id,
            issued_on: *issued_on,
        }
    }
}

pub struct RefreshTokenHash {
    bytes: Vec<u8>,
}

impl From<Vec<u8>> for RefreshTokenHash {
    fn from(value: Vec<u8>) -> Self {
        Self {
            bytes: value,
        }
    }
}

pub struct RefreshTokenBase64 {
    inner: String,
}

impl From<RefreshTokenHash> for RefreshTokenBase64 {
    fn from(value: RefreshTokenHash) -> Self {
        Self {
            inner: URL_SAFE_NO_PAD.encode(value.bytes.as_slice())
        }
    }
}

impl RefreshTokenBase64 {
    pub fn into_inner(self) -> String {
        self.inner
    }
}