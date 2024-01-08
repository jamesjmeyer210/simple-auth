use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::contact_info::ContactInfo;
use crate::email::Email;
use crate::ip_address::IpAddress;
use crate::password::Password;
use crate::realm::Realm;
use crate::role::Role;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    //pub email: Email,
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
            //email: Email::try_from("root@localhost.com").unwrap(),
            password: Password::try_from("password123").unwrap(),
            public_key: Vec::with_capacity(0),
            roles: Vec::with_capacity(0),
            realms: Vec::with_capacity(0),
            created_on: Utc::now(),
            deleted_on: None,
        }
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