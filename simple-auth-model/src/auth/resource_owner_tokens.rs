#[derive(Debug)]
pub struct ResourceOwnerTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}