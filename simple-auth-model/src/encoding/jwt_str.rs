use lazy_regex::regex_is_match;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

pub struct JwtStr<'s> {
    inner: &'s str,
}

impl JwtStr<'_> {
    fn is_jwt(s: &str) -> bool {
        regex_is_match!("^(([A-Za-z0-9_-]{2}){1,1028}[.]){2}([A-Za-z0-9_-]{1,256})$", s)
    }
}

impl <'s>TryFrom<&'s str> for JwtStr<'s> {
    type Error = ();

    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        match Self::is_jwt(s) {
            true => Ok(Self {
                inner: s
            }),
            false => Err(())
        }
    }
}

pub struct JwtStrParts<'s> {
    pub header: &'s str,
    pub claims: &'s str,
    pub sig: &'s str
}

impl <'s>JwtStr<'s> {
    pub fn into_parts(self) -> JwtStrParts<'s> {
        let mut split = self.inner.split('.');
        JwtStrParts {
            header: split.next().unwrap(),
            claims: split.next().unwrap(),
            sig: split.next().unwrap(),
        }
    }
}

pub struct JwtByteParts {
    pub header: Vec<u8>,
    pub claims: Vec<u8>,
    pub sig: Vec<u8>
}

impl From<JwtStrParts<'_>> for JwtByteParts {
    fn from(value: JwtStrParts) -> Self {
        Self {
            header: URL_SAFE_NO_PAD.decode(value.header).unwrap(),
            claims: URL_SAFE_NO_PAD.decode(value.claims).unwrap(),
            sig: URL_SAFE_NO_PAD.decode(value.sig).unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::encoding::jwt_str::{JwtByteParts, JwtStr};

    #[test]
    fn is_jwt_returns_true() {
        let jwts = vec![
            "eyJhbGdvIjoiU0hBMjU2IiwiX3R5cGUiOiJKV1QifQ\
            .eyJuYW1lIjoicm9vdCIsInJvbGVzIjpbIm1hc3RlciJdLCJyZWFsbXMiOlsicm9vdCJdLCJhdXRoX3RpbWUiOiIyMDI0LTAxLTI0VDAwOjM4OjI3Ljc4NjI2MDg0OFoifQ\
            .0z9gpqJIeISaqdQkhzl_Jj8Fi0yepYDD_5MvMqgB3bQ",
            "eyJhbGdvIjoiU0hBMjU2IiwiX3R5cGUiOiJKV1QifQ\
            .eyJuYW1lIjoicm9vdCIsInJvbGVzIjpbInJvb3QiXSwicmVhbG1zIjpbIm1hc3RlciJdLCJhdXRoX3RpbWUiOiIyMDI0LTAxLTI0VDA0OjE4OjA5Ljg1NjQ5NTk0NVoifQ\
            .fE5Cmm8J3AlSaweFuBtd90I8gpfbdPCmbgRq7TuMYKg",
            "eyJhbGdvIjoiU0hBMjU2IiwiX3R5cGUiOiJKV1QifQ\
            .eyJuYW1lIjoicm9vdCIsInJvbGVzIjpbInJvb3QiXSwicmVhbG1zIjpbIm1hc3RlciJdLCJhdXRoX3RpbWUiOiIyMDI0LTAxLTI0VDA0OjIwOjM4LjY0Njk5NjEzNVoifQ\
            .q3y4J7CDbtl390n7NTIejYpZPC3R8zs8m-B4GERJOyQ"
        ];

        for jwt in jwts.iter() {
            assert!(JwtStr::is_jwt(*jwt));
            let jwt_str = JwtStr::try_from(*jwt);
            assert!(jwt_str.is_ok());
            let parts = jwt_str.unwrap().into_parts();
            let _: JwtByteParts = parts.into();
        }
    }
}