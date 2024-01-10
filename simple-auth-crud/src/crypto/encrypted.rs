use std::marker::PhantomData;
use aes_gcm::{AeadCore, AeadInPlace, Aes256Gcm, KeyInit, KeySizeUser, TagSize};
use aes_gcm::aead::{Aead, OsRng, Nonce};
use aes_gcm::aes::cipher::ArrayLength;
use simple_auth_model::abs::AsBytes;
use crate::crypto::encryption_error::DecryptionError;
use crate::crypto::EncryptionError;
use crate::crypto::secret::Secret;

pub struct Encrypted<T: KeyInit + AeadCore + AeadInPlace> {
    bytes: Vec<u8>,
    nonce: Nonce<T>,
    _marker: PhantomData<T>
}

impl<T> Encrypted<T> where T: KeyInit + AeadCore + AeadInPlace {
    pub fn new(bytes: Vec<u8>, nonce: Nonce<T>) -> Self {
        Self {
            bytes,
            nonce,
            _marker: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /*fn get_data(&self) -> &[u8] {
        let l = self.bytes.len();
        self.bytes[self.nonce_size..l].as_ref()
    }*/

    pub fn decrypt<D>(&self, key: &Secret) -> Result<D,DecryptionError>
        where D : From<Vec<u8>>
    {
        let cipher = T::new_from_slice(key.as_bytes())
            .map_err(|e|DecryptionError::InvalidLength(e))?;

        /*let nonce = Nonce::<T>::from_slice(self.bytes[0..self.nonce_size].as_ref());
        let x = cipher.decrypt(nonce, key.as_bytes())
            .map_err(|e|DecryptionError::DecryptionFailed)?;
         */
        let x = cipher.decrypt(&self.nonce, self.bytes.as_ref())
            .map_err(|e|DecryptionError::DecryptionFailed)?;

        Ok(D::from(x))
    }
}

/*impl <T>From<Vec<u8>> for Encrypted<T> where T: KeyInit + AeadCore + AeadInPlace {
    fn from(value: Vec<u8>) -> Self {
        Self {
            bytes: value,
            _marker: Default::default()
        }
    }
}*/

pub fn encrypt<T>(data: &[u8], secret: &Secret) -> Result<Encrypted<T>,EncryptionError>
    where T: KeyInit + AeadCore + AeadInPlace
{
    let cipher = T::new_from_slice(secret.as_bytes())
        .map_err(|e|EncryptionError::InvalidLength(e))?;

    let nonce = T::generate_nonce(&mut OsRng);
    let enc = cipher.encrypt(&nonce, data)
        .map_err(|e|EncryptionError::EncryptionFailed)?;

    println!("Nonce: {:?}", &nonce);
    println!("Data: {:?}", enc);

    Ok(Encrypted::new(enc, nonce))
    //Ok(Encrypted::from(enc))
}

#[cfg(test)]
mod test {
    use aes_gcm::Aes256Gcm;
    use crate::crypto::encrypted::encrypt;
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
}