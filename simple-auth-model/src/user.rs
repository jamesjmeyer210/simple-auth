use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::contact_info::ContactInfo;
use crate::email::Email;
use crate::ip_address::IpAddress;
use crate::password::Password;
use crate::realm::Realm;
use crate::role::Role;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: Password,
    pub contact_info: Vec<ContactInfo>,
    pub public_key: Vec<u8>,
    pub roles: Vec<Role>,
    pub realms: Vec<Realm>,
    pub created_on: DateTime<Utc>,
    pub deleted_on: Option<DateTime<Utc>>
}

impl Default for User {
    fn default() -> Self {
        User {
            id: Uuid::new_v4(),
            name: String::from("root"),
            contact_info: vec![],
            password: Password::try_from("password123").unwrap(),
            public_key: Vec::with_capacity(0),
            roles: Vec::with_capacity(0),
            realms: Vec::with_capacity(0),
            created_on: Utc::now(),
            deleted_on: None,
        }
    }
}

impl User {
    pub fn with_realm(mut self, realm: Realm) -> Self {
        self.realms.push(realm);
        self
    }

    pub fn with_role(mut self, role: Role) -> Self {
        self.roles.push(role);
        self
    }

    pub fn with_contact_info(mut self, contact_info: ContactInfo) -> Self {
        self.contact_info.push(contact_info);
        self
    }
}

pub enum UserEventKind {
    Created,
    Deleted,
    PasswordChange,
    AddedRole(String),
    RemovedRole(String),
    AddedRealm(String),
    RemovedRealm(String),
    EmailChange(Email),
    EmailVerified(Email),
}

pub struct UserEvent {
    pub id: u32,
    pub user_id: Uuid,
    pub event: UserEventKind,
    pub ip_address: IpAddress,
    pub created_on: DateTime<Utc>,
}