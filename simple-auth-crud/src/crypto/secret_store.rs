use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::sync::Arc;
use aes_gcm::{aead::{KeyInit}, AeadCore, AeadInPlace, Aes256Gcm};
use simple_auth_model::abs::{AsBytes, AsJson};
use simple_auth_model::jwt::{JwtClaims, JwtHeader};
use crate::abs::table::Table;
use crate::crypto::encrypted::{encrypt, Encrypted};
use crate::crypto::EncryptionError;
use crate::crypto::secret::Secret;
use crate::crypto::sha_256_hash::Sha256Hash;
use crate::DbContext;
use crate::entity::SecretEntity;

struct SecretStoreInner {
    enc_key: Secret,
    sig_key: Secret,
}

impl Default for SecretStoreInner {
    fn default() -> Self {
        Self {
            enc_key: Secret::default(),
            sig_key: Secret::default(),
        }
    }
}

pub struct SecretStore {
    inner: Box<SecretStoreInner>
}

impl Default for SecretStore {
    fn default() -> Self {
        Self {
            inner: Box::new(SecretStoreInner::default())
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
        &self.inner.enc_key
    }

    fn set_enc_key(&mut self, secret: Secret) -> () {
        self.inner.enc_key = secret;
    }

    fn set_sig_key(&mut self, secret: Secret) -> () {
        self.inner.sig_key = secret;
    }

    /// Encrypts any `data` with the internal encryption key
    pub fn encrypt<T>(&self, data: &[u8]) -> Result<Encrypted<T>,EncryptionError>
        where T: KeyInit + AeadCore + AeadInPlace
    {
        encrypt(data, self.enc_key().as_bytes())
    }

    /// Returns a JWT signature
    pub fn sign_jwt(&self, header: &String, claims: &String) -> Vec<u8> {
        let hb = header.as_bytes();
        let cb = claims.as_bytes();
        let sb = self.inner.sig_key.as_bytes();

        let mut bytes = Vec::with_capacity(hb.len() + cb.len() + sb.len() + 2);
        for i in hb {
            bytes.push(*i);
        }
        bytes.push(b'.');

        for i in cb {
            bytes.push(*i);
        }
        bytes.push(b'.');

        for i in sb {
            bytes.push(*i);
        }

        Sha256Hash::from(bytes.as_slice()).into()
    }
}

pub(crate) struct SecretStoreBuilder<'r> {
    secrets: Arc<Table<'r, SecretEntity>>,
}

impl <'r>From<&DbContext<'r>> for SecretStoreBuilder<'r> {
    fn from(value: &DbContext<'r>) -> Self {
        Self {
            secrets: value.secrets.clone()
        }
    }
}

impl <'r>SecretStoreBuilder<'r> {
    pub async fn build(self) -> Result<SecretStore,sqlx::Error> {
        let mut store = SecretStore::default();

        if !self.secrets.contains("enc_key").await? {
            fs::write("enc_key",  store.enc_key().as_bytes()).unwrap();
            log::debug!("Wrote encryption key to \"enc_key\"");

            let encrypted = encrypt::<Aes256Gcm>(
                store.enc_key().as_bytes(),
                store.enc_key().as_bytes()
            ).unwrap();

            let _ = self.secrets.add(SecretEntity::new("enc_key", encrypted.into())).await?;
            log::debug!("Added enc_key to database");
        }
        else {
            let model = self.secrets.get("enc_key").await?;
            let encrypted = Encrypted::<Aes256Gcm>::try_from(model.value_enc).unwrap();

            let key = fs::read("enc_key").unwrap();
            let encrypted_key = Secret::try_from(key).unwrap();

            let decrypted_secret = encrypted.decrypt::<Secret>(&encrypted_key.as_bytes()).unwrap();

            store.set_enc_key(decrypted_secret);
        }

        if self.secrets.contains("sig_key").await? {
            let sig_key_enc = self.secrets.get("sig_key").await?;
            let encrypted = Encrypted::<Aes256Gcm>::try_from(sig_key_enc.value_enc).unwrap();
            let sig_key = encrypted.decrypt::<Secret>(store.enc_key().as_bytes()).unwrap();
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