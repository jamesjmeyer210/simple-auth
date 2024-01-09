use simple_auth_model::abs::AsBytes;

mod secret;
mod sha_256_hash;
mod secret_store;

pub trait Hash : AsBytes {
    fn len(&self) -> usize;
}

trait AsHash<T> where T: Hash {
    fn as_hash(&self) -> T;
}