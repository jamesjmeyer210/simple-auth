use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::uuid::Uuid;
use crate::abs::entity::Entity;
use crate::entity::{ContactInfoEntity, PasswordHash};

pub(crate) struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub password: Option<PasswordHash>,
    pub public_key: Vec<u8>,
    pub created_on: DateTime<Utc>,
    pub deleted_on: DateTime<Utc>
}

impl <'r>FromRow<'r, SqliteRow> for UserEntity {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            password: row.try_get("password")?,
            public_key: row.try_get("public_key")?,
            created_on: row.try_get("created_on")?,
            deleted_on: row.try_get("updated_on")?,
        })
    }
}

impl <'r>Entity<'r, Uuid> for UserEntity {
    fn primary_key(&self) -> &Uuid {
        &self.id
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }
}