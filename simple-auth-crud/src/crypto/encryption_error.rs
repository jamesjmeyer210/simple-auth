use aes_gcm::aes::cipher::InvalidLength;

pub enum EncryptionError {
    InvalidLength(InvalidLength),
    EncryptionFailed
}