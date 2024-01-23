use simple_auth_model::Password;
use simple_auth_model::abs::AsBytes;
use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore}
    },
    Argon2
};
use sqlx::{Database, Decode, Type};
use sqlx::database::{HasValueRef};
use sqlx::error::BoxDynError;

pub(crate) struct PasswordHash {
    salt: [u8;Self::SALT_LEN],
    hash: [u8;Self::HASH_LEN]
}

impl PasswordHash {
    pub(crate) const SALT_LEN: usize = 16;
    pub(crate) const HASH_LEN: usize = 32;

    pub(crate) fn hash(&self) -> &[u8] {
        self.hash.as_ref()
    }

    pub(crate) fn into_salt(self) -> [u8;Self::SALT_LEN] {
        self.salt
    }

    pub(crate) fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(Self::SALT_LEN + Self::HASH_LEN);
        let mut salt = self.salt.to_vec();
        let mut hash = self.hash.to_vec();
        vec.append(&mut salt);
        vec.append(&mut hash);
        vec
    }

    pub(crate) fn u8_from_bytes(password: &[u8], salt: &[u8;16]) -> Result<[u8;32],argon2::Error> {
        let argon2 = Argon2::default();
        let mut hash = [0u8;Self::HASH_LEN];
        argon2.hash_password_into(password, salt, &mut hash)?;
        Ok(hash)
    }

    pub(crate) fn verify(&self, password: &Password) -> Result<bool,argon2::Error> {
        let hash = Self::u8_from_bytes(password.as_bytes(), &self.salt)?;
        Ok(hash == self.hash)
    }
}

impl TryFrom<&Password> for PasswordHash {
    type Error = argon2::Error;

    fn try_from(value: &Password) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

impl TryFrom<&[u8]> for PasswordHash {
    type Error = argon2::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let argon2 = Argon2::default();
        let mut salt = [0u8;Self::SALT_LEN];
        OsRng.fill_bytes(&mut salt);

        let mut hash = [0u8;Self::HASH_LEN];
        argon2.hash_password_into(value, &salt, &mut hash)?;

        Ok(PasswordHash {
            salt,
            hash
        })
    }
}

impl<'r, DB: Database> Decode<'r, DB> for PasswordHash where &'r [u8]: Decode<'r, DB>
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&[u8] as Decode<DB>>::decode(value)?;

        let split = value.split_at(Self::SALT_LEN);

        Ok(PasswordHash {
            salt: <[u8;Self::SALT_LEN]>::try_from(split.0)?,
            hash: <[u8;Self::HASH_LEN]>::try_from(split.1)?
        })
    }
}

impl<DB: Database> Type<DB> for PasswordHash where [u8]: Type<DB> {
    fn type_info() -> DB::TypeInfo {
        <[u8]>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <[u8]>::compatible(ty)
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
        let h = PasswordHash::try_from(&p);
        assert!(h.is_ok())
    }

    #[test]
    fn verify_returns_true_when_passwords_match() {
        let p = Password::try_from("password123").unwrap();
        let hash = PasswordHash::try_from(&p).unwrap();

        let verify = hash.verify(&p);
        assert!(verify.is_ok());
        let verify = verify.unwrap();
        assert!(verify);
    }

    #[test]
    fn verify_returns_false_when_passwords_do_not_match() {
        let p = Password::try_from("password123").unwrap();
        let hash = PasswordHash::try_from(&p).unwrap();

        let p = Password::try_from("123password").unwrap();
        let verify = hash.verify(&p);
        assert!(verify.is_ok());
        let verify = verify.unwrap();
        assert!(!verify);
    }
}