use chrono::{DateTime, Utc};
use crate::role::Role;
use crate::user::User;

pub struct Realm {
    pub name: String,
    pub created_on: DateTime<Utc>,
    pub roles: Vec<Role>,
    pub users: Vec<User>,
}