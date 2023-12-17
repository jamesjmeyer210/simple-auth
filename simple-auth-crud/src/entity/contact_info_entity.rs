use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::uuid::Uuid;
use crate::abs::Entity;

pub(crate) struct ContactInfoEntity {
    pub user_id: Uuid,
    pub label: String,
    pub unique_id: Vec<u8>,
    pub enc: Vec<u8>,
    pub hash: Vec<u8>,
    pub verified: bool,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>
}

impl<'r> FromRow<'r, SqliteRow> for ContactInfoEntity {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            user_id: row.try_get("user_id")?,
            label: row.try_get("label")?,
            unique_id: row.try_get("unique_id")?,
            enc: row.try_get("enc")?,
            hash: row.try_get("hash")?,
            verified: row.try_get("verified")?,
            created_on: row.try_get("created_on")?,
            updated_on: row.try_get("updated_on")?,
        })
    }
}

impl <'r>Entity<'r, Vec<u8>> for ContactInfoEntity {
    fn primary_key(&self) -> &Vec<u8> {
        &self.unique_id
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }
}