use std::marker::PhantomData;
use aes_gcm::{AeadCore, Aes256Gcm, KeyInit};
use aes_gcm::aead::{Aead, OsRng};
use simple_auth_model::abs::AsBytes;
use crate::crypto::EncryptionError;
use crate::crypto::secret::Secret;

pub struct Encrypted<T> where T: KeyInit + AeadCore {
    bytes: Vec<u8>,
    _marker: PhantomData<T>
}

impl TryFrom<&Secret> for Encrypted<Aes256Gcm> {
    type Error = EncryptionError;

    fn try_from(value: &Secret) -> Result<Self,Self::Error> {
        let cipher = Aes256Gcm::new_from_slice(value.as_bytes())
            .map_err(|e|EncryptionError::InvalidLength(e))?;

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let enc = cipher.encrypt(&nonce, value.as_bytes())
            .map_err(|e|EncryptionError::EncryptionFailed)?;

        Ok(Self {
            bytes: enc,
            _marker: Default::default(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::crypto::encrypted::Encrypted;
    use crate::crypto::secret::Secret;

    #[test]
    fn try_from_returns_ok() {
        let s = Secret::default();
        let enc = Encrypted::try_from(&s);
        assert!(enc.is_ok());
    }
}