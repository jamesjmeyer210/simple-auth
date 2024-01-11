use std::fmt::{Debug, Formatter};
use std::io::Read;
use std::mem::size_of;
use aes_gcm::{AeadCore, AeadInPlace, KeyInit, KeySizeUser, TagSize};
use aes_gcm::aead::{Aead, OsRng, Nonce};
use aes_gcm::aes::cipher::{ArrayLength, InvalidLength};
use simple_auth_model::abs::AsBytes;
use crate::crypto::encryption_error::DecryptionError;
use crate::crypto::EncryptionError;
use crate::crypto::secret::Secret;

pub struct Encrypted<T: KeyInit + AeadCore + AeadInPlace> {
    bytes: Vec<u8>,
    nonce: Nonce<T>
}

impl <T>Clone for Encrypted<T>
    where T: KeyInit + AeadCore + AeadInPlace
{
    fn clone(&self) -> Self {
        let bytes = self.bytes.clone();
        let nonce = Nonce::<T>::clone_from_slice(self.nonce.as_slice());
        Self::new(bytes, nonce)
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
    fn new(bytes: Vec<u8>, nonce: Nonce<T>) -> Self {
        Self {
            bytes,
            nonce,
        }
    }

    pub fn len(&self) -> usize {
        self.nonce.len() + self.bytes.len()
    }

    pub fn decrypt<D>(&self, key: &Secret) -> Result<D,DecryptionError>
        where D : TryFrom<Vec<u8>>
    {
        let cipher = T::new_from_slice(key.as_bytes())
            .map_err(|e|DecryptionError::InvalidLength(e))?;

        let x = cipher.decrypt(&self.nonce, self.bytes.as_ref())
            .map_err(|e|DecryptionError::DecryptionFailed)?;

        Ok(D::try_from(x).map_err(|_|DecryptionError::DecryptionFailed)?)
    }
}

impl <T>Into<Vec<u8>> for Encrypted<T>
    where T: KeyInit + AeadCore + AeadInPlace
{
    fn into(self) -> Vec<u8> {
        let mut raw = Vec::with_capacity(size_of::<u8>() + self.len());
        raw.push(self.nonce.len() as u8);

        for i in self.nonce {
            raw.push(i)
        }

        for i in self.bytes {
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

        let nonce_len = value[0];
        if value.len() < size_of::<u8>() + (nonce_len as usize) {
            return Err(EncryptionError::InvalidLength(InvalidLength));
        }

        let n: Vec<u8> = value.iter()
            .skip(size_of::<u8>())
            .take(nonce_len as usize)
            .map(|x|*x)
            .collect();
        let nonce = Nonce::<T>::clone_from_slice(n.as_slice());

        let bytes: Vec<u8> = value.iter()
            .skip(size_of::<u8>() + nonce.len())
            .map(|x|*x)
            .collect();
        Ok(Encrypted::new(bytes, nonce))
    }
}

pub fn encrypt<T>(data: &[u8], secret: &Secret) -> Result<Encrypted<T>,EncryptionError>
    where T: KeyInit + AeadCore + AeadInPlace
{
    let cipher = T::new_from_slice(secret.as_bytes())
        .map_err(|e|EncryptionError::InvalidLength(e))?;

    let nonce = T::generate_nonce(&mut OsRng);
    let enc = cipher.encrypt(&nonce, data)
        .map_err(|e|EncryptionError::EncryptionFailed)?;

    Ok(Encrypted::new(enc, nonce))
}

#[cfg(test)]
mod test {
    use aes_gcm::Aes256Gcm;
    use crate::crypto::encrypted::{encrypt, Encrypted};
    use crate::crypto::secret::Secret;

    #[test]
    fn encrypt_returns_ok() {
        let s = Secret::default();
        let message = b"Lorem ipsum dolor set";
        let enc = encrypt::<Aes256Gcm>(message, &s);
        assert!(enc.is_ok());

        let enc = enc.unwrap();
        assert!(message.len() < enc.len());

        let dec = enc.decrypt::<Vec<u8>>(&s);

        assert!(dec.is_ok());
        let dec = dec.unwrap();

        assert_eq!(message.len(), dec.len());
    }

    #[test]
    fn try_from_returns_ok() {
        let s = Secret::default();
        let message = b"Encrypted message for a database";
        let enc_a = encrypt::<Aes256Gcm>(message, &s);
        assert!(enc_a.is_ok());

        let enc_a = enc_a.unwrap();
        let raw: Vec<u8> = enc_a.clone().into();
        assert!(raw.len() > 0);

        let enc_b = Encrypted::<Aes256Gcm>::try_from(raw);
        assert!(enc_b.is_ok());

        let enc_b = enc_b.unwrap();
        assert_eq!(enc_a, enc_b);
    }
}