use serde::Deserialize;

/// The configuration of security settings
#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub jwt_signature_scheme: JwtSignatureScheme
}

/// Defines the kind of signature scheme to use for signing JWTs
#[derive(Debug, Deserialize)]
pub enum JwtSignatureScheme {
    Secret,
    PublicKey,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            jwt_signature_scheme: JwtSignatureScheme::Secret,
        }
    }
}