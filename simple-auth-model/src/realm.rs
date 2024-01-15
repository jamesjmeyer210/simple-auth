use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::role::Role;
use crate::user::User;

#[derive(Debug, Serialize)]
pub struct Realm {
    pub name: String,
    pub created_on: DateTime<Utc>,
    pub roles: Vec<Role>,
    pub users: Vec<User>,
}

impl Default for Realm {
    fn default() -> Self {
        Self {
            name: String::from("master"),
            created_on: Utc::now(),
            roles: Vec::with_capacity(0),
            users: Vec::with_capacity(0)
        }
    }
}