use crate::crypto::secret::Secret;

struct SecretStoreInner {
    _enc_key: Secret,
    _sig_key: Secret,
}

impl Default for SecretStoreInner {
    fn default() -> Self {
        Self {
            _enc_key: Secret::default(),
            _sig_key: Secret::default(),
        }
    }
}

pub struct SecretStore {
    _inner: Box<SecretStoreInner>
}

impl Default for SecretStore {
    fn default() -> Self {
        Self {
            _inner: Box::new(SecretStoreInner::default())
        }
    }
}