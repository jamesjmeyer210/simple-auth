use simple_auth_model::chrono::{DateTime, Utc};
use crate::abs::entity::Entity;

pub(crate) struct Realm {
    pub name: String,
    pub created_on: DateTime<Utc>
}

impl Entity<String> for Realm {
    fn primary_key(&self) -> &String {
        &self.name
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }
}