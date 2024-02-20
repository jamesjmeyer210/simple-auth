use std::fmt::{Debug, Formatter};
use std::mem::size_of;
use aes_gcm::{AeadCore, AeadInPlace, KeyInit};
use aes_gcm::aead::{Aead, OsRng, Nonce};
use aes_gcm::aes::cipher::{InvalidLength};
use crate::error::encryption_error::DecryptionError;
use crate::crypto::{EncryptionError, PasswordHash};

pub struct Encrypted<T: KeyInit + AeadCore + AeadInPlace> {
    bytes: Vec<u8>,
    nonce: Nonce<T>,
    salt: [u8;16]
}

impl <T>Clone for Encrypted<T>
    where T: KeyInit + AeadCore + AeadInPlace
{
    fn clone(&self) -> Self {
        let bytes = self.bytes.clone();
        let nonce = Nonce::<T>::clone_from_slice(self.nonce.as_slice());
        Self::new(bytes, nonce, self.salt)
    }
}

impl <T>Debug for Encrypted<T>
    where T: KeyInit + AeadCore + AeadInPlace
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "bytes: {:?}", &self.bytes)?;
        writeln!(f, "nonce: {:?}", &self.nonce.as_slice())
    }
}

impl <T>PartialEq for Encrypted<T>
    where T: KeyInit + AeadCore + AeadInPlace
{
    fn eq(&self, other: &Self) -> bool {
        self.bytes.eq(&other.bytes) && self.nonce.as_slice().eq(other.nonce.as_slice())
    }
}

impl<T> Encrypted<T> where T: KeyInit + AeadCore + AeadInPlace {
    fn new(bytes: Vec<u8>, nonce: Nonce<T>, salt: [u8;16]) -> Self {
        Self {
            bytes,
            nonce,
            salt
        }
    }

    pub fn len(&self) -> usize {
        self.nonce.len() + self.bytes.len()
    }

    pub fn decrypt<D>(&self, key: &[u8]) -> Result<D,DecryptionError>
        where D : TryFrom<Vec<u8>>
    {
        let derivative = PasswordHash::u8_from_bytes(key, &self.salt)
            .map_err(DecryptionError::Argon2Error)?;

        let cipher = T::new_from_slice(&derivative)
            .map_err(DecryptionError::InvalidLength)?;

        let x = cipher.decrypt(&self.nonce, self.bytes.as_ref())
            .map_err(|_e|DecryptionError::DecryptionFailed)?;

        D::try_from(x).map_err(|_|DecryptionError::DecryptionFailed)
    }
}

impl <T>From<Encrypted<T>> for Vec<u8>
    where T: KeyInit + AeadCore + AeadInPlace
{
    fn from(val: Encrypted<T>) -> Self {
        let mut raw = Vec::with_capacity(size_of::<u8>() + val.len());
        raw.push(val.nonce.len() as u8);

        for i in val.nonce {
            raw.push(i);
        }

        for i in val.salt {
            raw.push(i);
        }

        for i in val.bytes {
            raw.push(i);
        }

        raw
    }
}

impl <T>TryFrom<Vec<u8>> for Encrypted<T>
    where T: KeyInit + AeadCore + AeadInPlace
{
    type Error = EncryptionError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() < size_of::<u8>() {
            return Err(EncryptionError::InvalidLength(InvalidLength));
        }
        // read the nonce length
        let nonce_len = value[0];
        if value.len() < size_of::<u8>() + (nonce_len as usize) {
            return Err(EncryptionError::InvalidLength(InvalidLength));
        }
        // read the nonce
        let n: Vec<u8> = value.iter()
            .skip(size_of::<u8>())
            .take(nonce_len as usize).copied()
            .collect();
        let nonce = Nonce::<T>::clone_from_slice(n.as_slice());
        // read the salt
        let s: Vec<u8> = value.iter()
            .skip(size_of::<u8>() + nonce_len as usize)
            .take(size_of::<[u8;16]>()).copied()
            .collect();
        let salt = <[u8;16]>::try_from(s).unwrap();
        // read the encrypted bytes
        let bytes: Vec<u8> = value.iter()
            .skip(size_of::<u8>() + nonce.len() + salt.len()).copied()
            .collect();
        Ok(Encrypted::new(bytes, nonce, salt))
    }
}

/// Encrypts the `data` with a `key` and returns an encrypted value of [Encrypted] if the encryption does not fail.
pub(crate) fn encrypt<T>(data: &[u8], key: &[u8]) -> Result<Encrypted<T>,EncryptionError>
    where T: KeyInit + AeadCore + AeadInPlace
{
    let derivative = PasswordHash::try_from(key)
        .map_err(EncryptionError::Argon2Error)?;

    let cipher = T::new_from_slice(derivative.hash())
        .map_err(EncryptionError::InvalidLength)?;

    let nonce = T::generate_nonce(&mut OsRng);
    let enc = cipher.encrypt(&nonce, data)
        .map_err(|_e|EncryptionError::EncryptionFailed)?;

    Ok(Encrypted::new(enc, nonce, derivative.into_salt()))
}

#[cfg(test)]
mod test {
    use aes_gcm::Aes256Gcm;
    use simple_auth_model::abs::AsBytes;
    use simple_auth_model::Password;
    use crate::crypto::encrypted::{encrypt, Encrypted};
    use crate::crypto::secret::Secret;

    #[test]
    fn encrypt_returns_ok() {
        let s = Secret::default();
        let message = b"Lorem ipsum dolor set";
        let enc = encrypt::<Aes256Gcm>(message, s.as_bytes());
        assert!(enc.is_ok());

        let enc = enc.unwrap();
        assert!(message.len() < enc.len());

        let dec = enc.decrypt::<Vec<u8>>(s.as_bytes());

        assert!(dec.is_ok());
        let dec = dec.unwrap();

        assert_eq!(message.len(), dec.len());
    }

    #[test]
    fn encrypt_returns_ok_for_password() {
        let p = Password::try_from("57db1253b68b6802b59a969f750fa32b60cb5cc8a3cb19b87dac28f541dc4e2a").unwrap();
        println!("bytes within password = {:?}", p.as_bytes().len());

        let data = b"user123@localhost.email";
        let enc = encrypt::<Aes256Gcm>(data, p.as_bytes());
        assert!(enc.is_ok());
    }

    #[test]
    fn try_from_returns_ok() {
        let s = Secret::default();
        let message = b"Encrypted message for a database";
        let enc_a = encrypt::<Aes256Gcm>(message, s.as_bytes());
        assert!(enc_a.is_ok());

        let enc_a = enc_a.unwrap();
        let raw: Vec<u8> = enc_a.clone().into();
        assert!(!raw.is_empty());

        let enc_b = Encrypted::<Aes256Gcm>::try_from(raw);
        assert!(enc_b.is_ok());

        let enc_b = enc_b.unwrap();
        assert_eq!(enc_a, enc_b);
    }
}