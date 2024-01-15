use serde::Serialize;
use crate::abs::AsBytes;
use crate::Email;

#[derive(Debug, Serialize)]
pub struct ContactInfo {
    pub verified: bool,
    pub label: String,
    pub value: ContactValue,
}

impl Default for ContactInfo {
    fn default() -> Self {
        Self {
            label: String::from("email"),
            value: ContactValue::Email(Email::try_from("root@localhost.com").unwrap()),
            verified: false,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ContactValue {
    Email(Email),
    Other(String)
}

impl AsBytes for ContactValue {
    fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Email(x) => x.as_bytes(),
            Self::Other(x) => x.as_bytes()
        }
    }
}