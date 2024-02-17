mod role_update;

use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::realm::Realm;

pub type RoleUpdate = role_update::RoleUpdate;

#[derive(Debug, Serialize)]
pub struct Role {
    pub name: String,
    pub max: Option<u32>,
    pub created_on: DateTime<Utc>,
    pub realm: String,
}

impl Default for Role {
    fn default() -> Self {
        Role {
            name: String::from("root"),
            max: Some(1),
            created_on: Utc::now(),
            realm: Realm::default().name,
        }
    }
}

impl Role {
    pub fn new(name: String, max: Option<u32>, realm: &Realm) -> Self
    {
        Self {
            name,
            max,
            realm: realm.name.clone(),
            created_on: Utc::now(),
        }
    }

    pub fn with_realm(mut self, realm: String) -> Self {
        self.realm = realm;
        self
    }
}