use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::ContactInfo;

pub struct FullUser {
    pub id: Uuid,
    pub name: String,
    pub created_on: DateTime<Utc>,
    pub contact_info: Vec<ContactInfo>,
    pub roles: Vec<String>,
    pub realms: Vec<String>,
}