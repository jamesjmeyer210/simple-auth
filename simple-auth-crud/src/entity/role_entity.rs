use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::Role;
use crate::abs::Entity;

pub(crate) struct RoleEntity {
    pub name: String,
    pub max: Option<u32>,
    pub created_on: DateTime<Utc>,
    pub deleted_on: Option<DateTime<Utc>>
}

impl From<&str> for RoleEntity {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
            max: None,
            created_on: Utc::now(),
            deleted_on: None
        }
    }
}

impl From<&Role> for RoleEntity {
    fn from(value: &Role) -> Self {
        Self {
            name: value.name.clone(),
            max: value.max,
            created_on: value.created_on,
            deleted_on: None,
        }
    }
}

impl <'r>FromRow<'r, SqliteRow> for RoleEntity {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            name: row.try_get("name")?,
            max: row.try_get("max")?,
            created_on: row.try_get("created_on")?,
            deleted_on: row.try_get("deleted_on")?
        })
    }
}

impl <'r>Entity<'r, String> for RoleEntity {
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