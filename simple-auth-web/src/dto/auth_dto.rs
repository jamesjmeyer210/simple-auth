use serde::Serialize;
use simple_auth_model::Password;

#[derive(Debug, Serialize)]
pub(crate) struct PasswordLoginDto {
    pub user_name: String,
    pub password: Password,
}