use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::ContactInfo;
use simple_auth_model::uuid::Uuid;

pub(crate) struct ContactInfoEntity {
    pub user_id: Uuid,
    pub label: String,
    pub enc: Vec<u8>,
    pub hash: Vec<u8>,
    pub verified: bool,
    pub created_on: DateTime<Utc>,
    pub deleted_on: Option<DateTime<Utc>>
}

impl<'r> FromRow<'r, SqliteRow> for ContactInfoEntity {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            user_id: row.try_get("user_id")?,
            label: row.try_get("label")?,
            enc: row.try_get("enc")?,
            hash: row.try_get("hash")?,
            verified: row.try_get("verified")?,
            created_on: row.try_get("created_on")?,
            deleted_on: row.try_get("deleted_on")?,
        })
    }
}

impl ContactInfoEntity {
    pub fn new(contact: &ContactInfo, user_id: &Uuid) -> Self
    {
        Self {
            user_id: user_id.clone(),
            label: contact.label.clone(),
            enc: vec![],
            hash: vec![],
            verified: false,
            created_on: Utc::now(),
            deleted_on: None,
        }
    }
}