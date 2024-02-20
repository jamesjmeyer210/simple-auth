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

#[cfg(test)]
mod test {
    use super::Hasher;
    use super::Argon2;

    #[test]
    fn verify_returns_true() {
        let d = b"Hello SHA256!";
        let x = Hasher::hash(d);
        assert!(Argon2::verify(&x, d));
    }
}