use sha2::{Digest, Sha256};
use simple_auth_model::abs::AsBytes;
use crate::crypto::abs::Hash;

pub(crate) struct Sha256Hash {
    _hash: Vec<u8>
}

impl From<&[u8]> for Sha256Hash {
    fn from(value: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(value);
        Self {
            _hash: hasher.finalize().to_vec()
        }
    }
}

impl From<Sha256Hash> for Vec<u8> {
    fn from(val: Sha256Hash) -> Self {
        val._hash
    }
}

impl AsBytes for Sha256Hash {
    fn as_bytes(&self) -> &[u8] {
        self._hash.as_slice()
    }
}

impl Hash for Sha256Hash {
    fn len(&self) -> usize {
        self._hash.len()
    }
}