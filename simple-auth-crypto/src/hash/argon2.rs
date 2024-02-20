use super::{Hash, Hasher};
use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore}
    }
};

pub struct Argon2;

impl Argon2 {
    fn hash(salt: &[u8], digest: &[u8]) -> Hash<Self> {
        let mut h = Hash::<Self>::empty();

        let argon2 = argon2::Argon2::default();
        argon2.hash_password_into(digest, salt, h.hash.as_mut_slice()).unwrap();

        h.salt.copy_from_slice(salt);
        h

        /*let argon2 = argon2::Argon2::default();
        let mut hash = [0u8;Self::HASH_LEN];
        argon2.hash_password_into(digest, &salt, &mut hash).unwrap();

        Hash {
            salt: salt.to_vec(),
            hash: hash.to_vec(),
            _phantom: PhantomData::default()
        }*/
    }
}

impl Hasher for Argon2 {
    const SALT_LEN: usize = 16;
    const HASH_LEN: usize = 32;

    fn hash(digest: &[u8]) -> Hash<Self> {
        let mut salt = [0u8;Self::SALT_LEN];
        OsRng.fill_bytes(&mut salt);
        Self::hash(&salt, digest)
    }

    fn verify(src: &Hash<Self>, digest: &[u8]) -> bool {
        let other = Self::hash(src.salt.as_slice(), digest);
        src.hash == other.hash
    }
}