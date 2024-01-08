
// TODO: implement custom Debug that protects the password
#[derive(Debug)]
pub struct Password {
    _inner: String,
}

impl TryFrom<&str> for Password {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            Err("password must be at least 8 characters")
        }
        else {
            Ok(Password {
                _inner: value.to_string()
            })
        }
    }
}

impl Password {
    pub fn into_inner(self) -> String {
        self._inner
    }

    pub fn as_bytes(&self) -> &[u8] {
        self._inner.as_bytes()
    }
}