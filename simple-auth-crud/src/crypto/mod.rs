use simple_auth_model::abs::AsBytes;

mod secret;
mod sha_256_hash;
mod secret_store;
mod encrypted;
mod encryption_error;
mod abs;

pub type EncryptionError = encryption_error::EncryptionError;
pub type SecretStore = secret_store::SecretStore;
pub type Secret = secret::Secret;
pub(crate) type SecretStoreBuilder<'r> = secret_store::SecretStoreBuilder<'r>;