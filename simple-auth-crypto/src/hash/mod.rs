use std::marker::PhantomData;

mod argon2;
mod sha256;

pub type Sha256 = sha256::Sha256;
pub type Argon2 = argon2::Argon2;

pub trait Hasher: Sized {
    const SALT_LEN: usize;
    const HASH_LEN: usize;

    fn hash(digest: &[u8]) -> Hash<Self>;

    fn verify(src: &Hash<Self>, digest: &[u8]) -> bool;
}

pub struct Hash<T: Hasher>
{
    pub(crate) salt: Vec<u8>,
    pub(crate) hash: Vec<u8>,
    _phantom: PhantomData<T>
}

impl<T> Hash<T> where T: Hasher
{
    fn empty() -> Self {
        Self {
            salt: vec![0u8;T::SALT_LEN],
            hash: vec![0u8;T::HASH_LEN],
            _phantom: PhantomData::default()
        }
    }

    fn as_bytes(&self) -> &[u8] {
        self.hash.as_slice()
    }

    fn len(&self) -> usize {
        self.hash.len()
    }
}