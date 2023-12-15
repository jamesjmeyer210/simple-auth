use chrono::{DateTime, Utc};

pub struct Role {
    pub name: String,
    pub max: Option<i32>,
    pub created_on: DateTime<Utc>,
}

impl Default for Role {
    fn default() -> Self {
        Role {
            name: String::from("root"),
            max: Some(1),
            created_on: Utc::now(),
        }
    }
}