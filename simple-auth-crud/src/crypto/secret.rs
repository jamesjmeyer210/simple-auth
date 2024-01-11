use std::ops::{BitXor, Deref};
use aes_gcm::aead::OsRng;
use aes_gcm::{Aes256Gcm, KeyInit};
use sha2::{Sha256, Digest};
use sha2::digest::{DynDigest, FixedOutput};
use simple_auth_model::abs::AsBytes;
use crate::crypto::AsHash;
use crate::crypto::sha_256_hash::Sha256Hash;

pub(crate) struct Secret {
    _key: [u8;32]
}

impl Default for Secret {
    fn default() -> Self {
        let key = Aes256Gcm::generate_key(OsRng);
        Self {
            _key: key.try_into().unwrap()
        }
    }
}

impl TryFrom<Vec<u8>> for Secret {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self {
            _key: value.try_into()?
        })
    }
}

impl AsHash<Sha256Hash> for Secret {
    fn as_hash(&self) -> Sha256Hash {
        Sha256Hash::from(self._key.as_ref())
    }
}

impl AsBytes for Secret {
    fn as_bytes(&self) -> &[u8] {
        self._key.as_ref()
    }
}

#[cfg(test)]
mod test {
    use simple_auth_model::abs::AsBytes;
    use crate::crypto::{AsHash, Hash};
    use super::Secret;

    #[test]
    fn default_test() {
        let s = Secret::default();
        let h = s.as_hash();
        assert_eq!(32, h.len());
    }

    #[test]
    fn try_from_returns_err() {
        let vec = vec![1,2,3];

        let x = Secret::try_from(vec);
        assert!(x.is_err());
    }

    #[test]
    fn try_from_returns_ok() {
        let s = Secret::default();
        let vec = s.as_bytes().to_vec();

        let x = Secret::try_from(vec);
        assert!(x.is_ok());
    }
}