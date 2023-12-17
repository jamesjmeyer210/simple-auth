use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use crate::abs::Entity;

pub(crate) struct Realm {
    pub name: String,
    pub created_on: DateTime<Utc>
}

impl <'r>FromRow<'r, SqliteRow> for Realm {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            name: row.try_get("name")?,
            created_on: row.try_get("created_on")?,
        })
    }
}

impl <'r>Entity<'r, String> for Realm {
    fn primary_key(&self) -> &String {
        &self.name
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }
}