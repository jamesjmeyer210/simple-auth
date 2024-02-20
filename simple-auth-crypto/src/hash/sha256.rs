use std::marker::PhantomData;
use super::{Hash, Hasher};
use sha2::{Digest};

pub struct Sha256;

impl Hasher for Sha256 {
    const SALT_LEN: usize = 0;
    const HASH_LEN: usize = 32;

    fn hash(digest: &[u8]) -> Hash<Sha256> {
        let mut hasher = sha2::Sha256::new();
        hasher.update(digest);

        Hash {
            salt: vec![0u8;Self::SALT_LEN],
            hash: hasher.finalize().to_vec(),
            _phantom: PhantomData::default()
        }
    }

    fn verify(src: &Hash<Self>, digest: &[u8]) -> bool {
        let other = Self::hash(digest);
        src.hash == other.hash
    }
}