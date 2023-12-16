use simple_auth_model::chrono::{DateTime, Utc};
use crate::abs::entity::Entity;

pub(crate) struct RoleEntity {
    pub name: String,
    pub max: Option<u32>,
    pub created_on: DateTime<Utc>
}

impl Entity<String> for RoleEntity {
    fn primary_key(&self) -> &String {
        &self.name
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }
}