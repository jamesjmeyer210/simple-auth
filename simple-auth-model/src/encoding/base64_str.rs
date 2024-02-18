
use lazy_regex::regex_is_match;

pub struct Base64Str<'s> {
    inner: &'s str
}

impl Base64Str<'_> {
    fn is_base64(s: &str) -> bool {
        regex_is_match!("^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$", s)
    }
}

impl <'s>TryFrom<&'s str> for Base64Str<'s> {
    type Error = ();

    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        match Self::is_base64(s) {
            true => Ok(Self {
                inner: s,
            }),
            false => Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::encoding::base64_str::Base64Str;

    fn is_base64_returns_true() {
        let input = ["aGVsbG8K",
            "aGVsbG8gd29ybGQK",
            "eyJhbGdvIjoiU0hBMjU2IiwiX3R5cGUiOiJKV1QifQ",
            "eyJuYW1lIjoicm9vdCIsInJvbGVzIjpbIm1hc3RlciJdLCJyZWFsbXMiOlsicm9vdCJdLCJhdXRoX3RpbWUiOiIyMDI0LTAxLTI0VDAwOjM4OjI3Ljc4NjI2MDg0OFoifQ",
            "0z9gpqJIeISaqdQkhzl_Jj8Fi0yepYDD_5MvMqgB3bQ"];

        for i in input.iter() {
            assert!(Base64Str::is_base64(i));
        }
    }
}