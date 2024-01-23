use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::realm::Realm;

#[derive(Debug, Serialize)]
pub struct Role {
    pub name: String,
    pub max: Option<u32>,
    pub created_on: DateTime<Utc>,
    pub realms: Vec<Realm>,
}

impl Default for Role {
    fn default() -> Self {
        Role {
            name: String::from("root"),
            max: Some(1),
            created_on: Utc::now(),
            realms: Vec::with_capacity(0)
        }
    }
}

impl Role {
    pub fn new(name: String, max: Option<u32>, realms: Vec<Realm>) -> Self
    {
        Self {
            name,
            max,
            realms,
            created_on: Utc::now(),
        }
    }

    pub fn with_realm(mut self, realm: Realm) -> Self {
        self.realms.push(realm);
        self
    }
}