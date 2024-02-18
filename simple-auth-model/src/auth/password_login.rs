use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordLogin {
    pub user_name: String,
    pub password: String,
}