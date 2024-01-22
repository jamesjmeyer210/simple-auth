use simple_auth_model::abs::AsBytes;

pub trait Hash : AsBytes {
    fn len(&self) -> usize;
}

pub trait AsHash<T> where T: Hash {
    fn as_hash(&self) -> T;
}