use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::sync::Arc;
use aes_gcm::{aead::{KeyInit}, AeadCore, AeadInPlace, Aes256Gcm};
use simple_auth_model::abs::AsBytes;
use crate::abs::table::Table;
use crate::crypto::encrypted::{encrypt, Encrypted};
use crate::crypto::EncryptionError;
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

impl Debug for SecretStore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", TypeId::of::<Self>())
    }
}

impl SecretStore {
    fn enc_key(&self) -> &Secret {
        &self._inner._enc_key
    }

    fn set_enc_key(&mut self, secret: Secret) -> () {
        self._inner._enc_key = secret;
    }

    fn set_sig_key(&mut self, secret: Secret) -> () {
        self._inner._sig_key = secret;
    }
    pub fn encrypt<T>(&self, data: &[u8]) -> Result<Encrypted<T>,EncryptionError>
        where T: KeyInit + AeadCore + AeadInPlace
    {
        encrypt(data, self.enc_key().as_bytes())
    }
}

pub(crate) struct SecretStoreBuilder<'r> {
    _secrets: Arc<Table<'r, SecretEntity>>,
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
            fs::write("enc_key",  store.enc_key().as_bytes()).unwrap();
            log::debug!("Wrote encryption key to \"enc_key\"");

            let encrypted = encrypt::<Aes256Gcm>(
                store.enc_key().as_bytes(),
                store.enc_key().as_bytes()
            ).unwrap();

            let _ = self._secrets.add(SecretEntity::new("enc_key", encrypted.into())).await?;
            log::debug!("Added enc_key to database");
        }
        else {
            let model = self._secrets.get("enc_key").await?;
            let encrypted = Encrypted::<Aes256Gcm>::try_from(model.value_enc).unwrap();

            let key = fs::read("enc_key").unwrap();
            let encrypted_key = Secret::try_from(key).unwrap();

            let decrypted_secret = encrypted.decrypt::<Secret>(&encrypted_key).unwrap();

            store.set_enc_key(decrypted_secret);
        }

        if self._secrets.contains("sig_key").await? {
            let sig_key_enc = self._secrets.get("sig_key").await?;
            let encrypted = Encrypted::<Aes256Gcm>::try_from(sig_key_enc.value_enc).unwrap();
            let sig_key = encrypted.decrypt::<Secret>(store.enc_key()).unwrap();
            store.set_sig_key(sig_key);
        }

        Ok(store)
    }
}

#[cfg(test)]
mod test {
    use crate::DbContext;
    use super::SecretStoreBuilder;

    #[sqlx::test]
    async  fn build_returns_ok() {
        let db = DbContext::in_memory().await.unwrap();
        let builder: SecretStoreBuilder = (&db).into();

        let store = builder.build().await;
        assert!(store.is_ok());
    }
}