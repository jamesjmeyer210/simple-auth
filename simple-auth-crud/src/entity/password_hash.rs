use simple_auth_model::Password;
use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore}
    },
    Argon2
};

pub(crate) struct PasswordHash {
    _inner: [u8;32]
}

impl TryFrom<Password> for PasswordHash {
    type Error = argon2::Error;

    fn try_from(value: Password) -> Result<Self, Self::Error> {
        let argon2 = Argon2::default();
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);

        let mut bytes = [0u8; 32];
        argon2.hash_password_into(value.as_bytes(), &salt, &mut bytes)?;
        Ok(PasswordHash {
            _inner: bytes
        })
    }
}

#[cfg(test)]
mod test {
    use simple_auth_model::Password;
    use super::PasswordHash;

    #[test]
    fn try_from_returns_ok() {
        let p = Password::try_from("password123");
        assert!(p.is_ok());

        let p = p.unwrap();
        let h = PasswordHash::try_from(p);
        assert!(h.is_ok())
    }
}