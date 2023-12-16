use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::uuid::Uuid;
use crate::abs::entity::Entity;

pub(crate) struct ContactInfoEntity {
    pub user_id: Uuid,
    pub label: String,
    pub unique_id: Vec<u8>,
    pub enc: Vec<u8>,
    pub hash: Vec<u8>,
    pub verified: bool,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>
}

impl Entity<Vec<u8>> for ContactInfoEntity {
    fn primary_key(&self) -> &Vec<u8> {
        &self.unique_id
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }
}