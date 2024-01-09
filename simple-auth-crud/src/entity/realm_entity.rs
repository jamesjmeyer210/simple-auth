use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::Realm;

#[derive(Debug)]
pub(crate) struct RealmEntity {
    pub name: String,
    pub created_on: DateTime<Utc>,
    pub deleted_on: Option<DateTime<Utc>>
}

impl From<RealmEntity> for Realm {
    fn from(value: RealmEntity) -> Self {
        Self {
            name: value.name,
            created_on: value.created_on,
            roles: Vec::with_capacity(0),
            users: Vec::with_capacity(0),
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