use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::Role;

pub(crate) struct RoleEntity {
    pub name: String,
    pub max: Option<u32>,
    pub realm_id: String,
    pub created_on: DateTime<Utc>,
    pub deleted_on: Option<DateTime<Utc>>
}

impl From<&Role> for RoleEntity {
    fn from(value: &Role) -> Self {
        Self {
            name: value.name.clone(),
            max: value.max,
            realm_id: value.realm.clone(),
            created_on: value.created_on,
            deleted_on: None,
        }
    }
}

impl Into<Role> for RoleEntity {
    fn into(self) -> Role {
        Role {
            name: self.name,
            max: self.max,
            created_on: self.created_on,
            realm: self.realm_id,
        }
    }
}

impl <'r>FromRow<'r, SqliteRow> for RoleEntity {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            name: row.try_get("name")?,
            max: row.try_get("max")?,
            realm_id: row.try_get("realm_id")?,
            created_on: row.try_get("created_on")?,
            deleted_on: row.try_get("deleted_on")?
        })
    }
}