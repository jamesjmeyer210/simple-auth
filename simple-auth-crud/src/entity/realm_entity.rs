use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::Realm;
use crate::abs::Entity;

#[derive(Debug)]
pub(crate) struct RealmEntity {
    pub name: String,
    pub created_on: DateTime<Utc>,
    pub deleted_on: Option<DateTime<Utc>>
}

impl Into<Realm> for RealmEntity {
    fn into(self) -> Realm {
        Realm {
            name: self.name,
            created_on: self.created_on,
            roles: Vec::with_capacity(0),
            users: Vec::with_capacity(0)
        }
    }
}

impl From<&str> for RealmEntity {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
            created_on: Utc::now(),
            deleted_on: None
        }
    }
}

impl <'r>FromRow<'r, SqliteRow> for RealmEntity {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            name: row.try_get(0)?,
            created_on: row.try_get(1)?,
            deleted_on: row.try_get(2)?
        })
    }
}

impl <'r>Entity<'r, String> for RealmEntity {
    fn primary_key(&self) -> &String {
        &self.name
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }

    fn is_deleted(&self) -> bool {
        self.deleted_on.is_some()
    }
}