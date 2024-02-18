use lazy_regex::regex_is_match;
use serde::Serialize;
use crate::abs::{AsBytes, IsValid};

#[derive(Debug, Serialize)]
pub struct Email {
    _inner: String,
}

impl TryFrom<&str> for Email {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Self::is_valid(value) {
            true => Ok(Email {
                _inner: value.to_string()
            }),
            _ => Err("Invalid email address"),
        }
    }
}

impl IsValid<&str> for Email {
    fn is_valid(value: &str) -> bool {
        regex_is_match!(r"^([a-zA-Z0-9!#$%&'*+\-/=?^_`{|}~\.]{1,256}@)([a-z0-9]{1,256})([.][a-z0-9]{1,64}){1,8}$", value)
    }
}

impl AsBytes for Email {
    fn as_bytes(&self) -> &[u8] {
        self._inner.as_bytes()
    }
}

impl Email {
    pub fn into_inner(self) -> String {
        self._inner
    }
}

#[cfg(test)]
mod test {
    use super::Email;

    #[test]
    fn try_from_test() {
        let emails = ["root@localhost.com",
            "roo123@localhost.com",
            "root@admin.localhost.org",
            "root+admin@email.localhost.net",
            "r00!/admin@localhost.127.0.0.1",
            "Aryanna.Hodkiewicz@yahoo.com"];

        for email in emails.iter() {
            let x = Email::try_from(*email);
            assert!(x.is_ok())
        }
    }
}