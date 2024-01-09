use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};

pub(crate) struct SecretEntity {
    pub key: String,
    pub value_enc: Vec<u8>,
    pub expires_on: Option<DateTime<Utc>>
}

impl <'r>FromRow<'r, SqliteRow> for SecretEntity {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            key: row.try_get("key")?,
            value_enc: row.try_get("value_enc")?,
            expires_on: row.try_get("expires_on")?
        })
    }
}