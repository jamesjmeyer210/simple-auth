use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::uuid::Uuid;
use crate::abs::entity::Entity;
use crate::entity::{ContactInfoEntity, PasswordHash};

pub(crate) struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub password: Option<PasswordHash>,
    pub public_key: Vec<u8>,
    pub contact_info: Vec<ContactInfoEntity>,
    pub created_on: DateTime<Utc>,
    pub deleted_on: DateTime<Utc>
}

impl Entity<Uuid> for UserEntity {
    fn primary_key(&self) -> &Uuid {
        &self.id
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }
}