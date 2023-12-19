use simple_auth_model::Password;
use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore}
    },
    Argon2
};
use sqlx::{Database, Decode, Encode, Type, Value, ValueRef};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;

pub(crate) struct PasswordHash {
    _inner: [u8;32]
}

// TODO: Implement explicit decode
impl PasswordHash {
    pub fn as_bytes(&self) -> &[u8] {
        self._inner.as_ref()
    }
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

impl<'r, DB: Database> Decode<'r, DB> for PasswordHash where &'r [u8]: Decode<'r, DB>
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&[u8] as Decode<DB>>::decode(value)?;

        Ok(PasswordHash {
            _inner: <[u8;32]>::try_from(value)?
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
        let h = PasswordHash::try_from(p);
        assert!(h.is_ok())
    }
}