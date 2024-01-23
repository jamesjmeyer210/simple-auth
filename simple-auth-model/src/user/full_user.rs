use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::ContactInfo;
use crate::user::PartialUser;

#[derive(Debug, Serialize)]
pub struct FullUser {
    pub id: Uuid,
    pub name: String,
    pub created_on: DateTime<Utc>,
    pub contact_info: Vec<ContactInfo>,
    pub roles: Vec<String>,
    pub realms: Vec<String>,
}

impl FullUser {
    pub fn new(user: PartialUser, contact_info: Vec<ContactInfo>, roles: Vec<String>, realms: Vec<String>) -> Self {
        Self {
            id: user.id,
            name: user.name,
            created_on: user.created_on,
            contact_info,
            realms,
            roles
        }
    }
}