use serde::{Deserialize};
use simple_auth_model::Password;

#[derive(Debug, Deserialize)]
pub(crate) struct PasswordLoginDto {
    pub user_name: String,
    pub password: String,
}