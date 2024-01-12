use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::{Password, User};
use simple_auth_model::uuid::Uuid;
use crate::entity::{PasswordHash};

pub(crate) struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub password: Option<PasswordHash>,
    pub created_on: DateTime<Utc>,
    pub deleted_on: Option<DateTime<Utc>>,
}

impl Default for UserEntity {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("root"),
            password: Password::try_from("password123")
                .map(|p|PasswordHash::try_from(&p))
                .unwrap()
                .map(|h|Some(h))
                .unwrap(),
            created_on: Utc::now(),
            deleted_on: None
        }
    }
}

impl From<&User> for UserEntity {
    fn from(value: &User) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            password: PasswordHash::try_from(&value.password).ok(),
            created_on: value.created_on,
            deleted_on: value.deleted_on
        }
    }
}

impl <'r>FromRow<'r, SqliteRow> for UserEntity {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get(0)?,
            name: row.try_get(1)?,
            password: row.try_get(2)?,
            created_on: row.try_get(3)?,
            deleted_on: row.try_get(4)?,
        })
    }
}