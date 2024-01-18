use serde::{Deserialize};
use simple_auth_model::{ContactInfo, ContactValue};

#[derive(Debug, Deserialize)]
pub(crate) struct AddUserDto {
    pub name: String,
    pub password1: String,
    pub password2: String,
    pub realm: String,
    pub role: String,
    pub contact: AddUserContactDto,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AddUserContactDto {
    pub label: String,
    pub value: String,
}

impl TryInto<ContactInfo> for AddUserContactDto {
    type Error = &'static str;

    fn try_into(self) -> Result<ContactInfo, Self::Error> {
        Ok(ContactInfo {
            verified: false,
            value: ContactValue::try_new(&self.label, &self.value)?,
            label: self.label,
        })
    }
}