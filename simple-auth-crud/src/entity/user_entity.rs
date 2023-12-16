use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::uuid::Uuid;

pub(crate) struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub password: Vec<u8>,
    pub public_key: Vec<u8>,
    // Email address encrypted with the user's password
    pub email_enc: Vec<u8>,
    // Hash of the email address
    pub email_hash: Vec<u8>,
    pub created_on: DateTime<Utc>,
    pub deleted_on: DateTime<Utc>
}