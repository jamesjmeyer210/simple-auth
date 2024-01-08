use crate::Email;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ContactValue {
    Email(Email),
    Other(String)
}