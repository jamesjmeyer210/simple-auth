use serde::{Deserialize};


#[derive(Debug, Deserialize)]
pub(crate) struct PasswordLoginDto {
    pub user_name: String,
    pub password: String,
}