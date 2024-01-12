use simple_auth_model::abs::AsBytes;

pub trait Hash : AsBytes {
    fn len(&self) -> usize;
}

pub trait AsHash<T> where T: Hash {
    fn as_hash(&self) -> T;
}

impl <T,H>AsHash<H> for &T where T: AsBytes, H: for<'a> From<&'a [u8]> + Hash {
    fn as_hash(&self) -> H {
        H::from(self.as_bytes())
    }
}