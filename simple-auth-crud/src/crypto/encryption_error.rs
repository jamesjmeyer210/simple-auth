use aes_gcm::aes::cipher::InvalidLength;

#[derive(Debug)]
pub enum EncryptionError {
    InvalidLength(InvalidLength),
    EncryptionFailed,
    DecodingFailed,
    Argon2Error(argon2::Error)
}

#[derive(Debug)]
pub enum DecryptionError {
    InvalidLength(InvalidLength),
    DecryptionFailed
}