use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::Password;
use simple_auth_model::uuid::Uuid;
use crate::abs::Entity;
use crate::entity::{PasswordHash};

pub(crate) struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub password: Option<PasswordHash>,
    pub created_on: DateTime<Utc>,
    pub deleted_on: Option<DateTime<Utc>>
}

impl Default for UserEntity {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("root"),
            password: Password::try_from("password123")
                .map(|p|PasswordHash::try_from(p))
                .unwrap()
                .map(|h|Some(h))
                .unwrap(),
            created_on: Utc::now(),
            deleted_on: None,
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

impl <'r>Entity<'r, Uuid> for UserEntity {
    fn primary_key(&self) -> &Uuid {
        &self.id
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }

    fn is_deleted(&self) -> bool {
        self.deleted_on.is_some()
    }
}