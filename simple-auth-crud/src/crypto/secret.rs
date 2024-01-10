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

/*impl Encrypt<Aes256Gcm> for Secret {
    type Output = Encrypted<Aes256Gcm>;
}*/

#[cfg(test)]
mod test {
    use crate::crypto::{AsHash, Hash};
    use super::Secret;

    #[test]
    fn default_test() {
        let s = Secret::default();
        let h = s.as_hash();
        assert_eq!(32, h.len());
    }
}