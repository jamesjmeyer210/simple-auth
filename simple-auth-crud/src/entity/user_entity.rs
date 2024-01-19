use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::{Password, User};
use simple_auth_model::user::PartialUser;
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
            password: value.password
                .as_ref()
                .map(|p|PasswordHash::try_from(p))
                .and_then(|x|x.ok()),
            created_on: value.created_on,
            deleted_on: value.deleted_on
        }
    }
}

impl Into<User> for UserEntity {
    fn into(self) -> User {
        User {
            id: self.id,
            name: self.name,
            password: None,
            contact_info: Vec::with_capacity(0),
            public_key: Vec::with_capacity(0),
            roles: Vec::with_capacity(0),
            realms: Vec::with_capacity(0),
            created_on: self.created_on,
            deleted_on: None,
        }
    }
}

impl Into<PartialUser> for UserEntity {
    fn into(self) -> PartialUser {
        PartialUser {
            id: self.id,
            name: self.name,
            created_on: self.created_on,
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