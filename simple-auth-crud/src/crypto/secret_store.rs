use std::fs;
use std::sync::Arc;
use aes_gcm::{
    aead::{AeadCore, OsRng, AeadInPlace, KeyInit, heapless::Vec},
    Aes256Gcm, Nonce
};
use aes_gcm::aead::Aead;
use simple_auth_model::abs::AsBytes;
use crate::abs::table::Table;
use crate::crypto::secret::Secret;
use crate::DbContext;
use crate::entity::SecretEntity;

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

pub(crate) struct SecretStore {
    _inner: Box<SecretStoreInner>
}

impl Default for SecretStore {
    fn default() -> Self {
        Self {
            _inner: Box::new(SecretStoreInner::default())
        }
    }
}

impl SecretStore {
    fn enc_key(&self) -> &Secret {
        &self._inner._enc_key
    }
}

pub(crate) struct SecretStoreBuilder<'r> {
    _secrets: Arc<Table<'r, SecretEntity>>
}

impl <'r>From<&DbContext<'r>> for SecretStoreBuilder<'r> {
    fn from(value: &DbContext<'r>) -> Self {
        Self {
            _secrets: value.secrets.clone()
        }
    }
}

impl <'r>SecretStoreBuilder<'r> {
    pub async fn build(self) -> Result<SecretStore,sqlx::Error> {
        let mut store = SecretStore::default();

        if !self._secrets.contains("enc_key").await? {
            fs::write("enc_key",  store._inner._enc_key.as_bytes()).unwrap();
            log::debug!("Wrote encryption key to \"enc_key\"");

            let cipher = Aes256Gcm::new_from_slice(store.enc_key().as_bytes()).unwrap();
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

            let enc = cipher.encrypt(&nonce, store.enc_key().as_bytes()).unwrap();
            let _ = self._secrets.add(SecretEntity::new("enc_key", enc)).await?;
            log::debug!("Added enc_key to database");
        }
        else {
            let model = self._secrets.get("enc_key").await?;
            let key = fs::read("enc_key").unwrap();
            let nonce = &model.value_enc[0..98];

            let cipher = Aes256Gcm::new_from_slice(key.as_slice()).unwrap();
            let nonce = Nonce::from_slice(nonce);
            let enc_key = cipher.decrypt(nonce, &*model.value_enc).unwrap();

        }

        Ok(store)
    }
}