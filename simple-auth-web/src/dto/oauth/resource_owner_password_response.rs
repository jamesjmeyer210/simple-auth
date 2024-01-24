use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ResourceOwnerPasswordResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: u32,
}

impl ResourceOwnerPasswordResponse {
    pub(crate) fn bearer() -> Self {
        Self {
            access_token: "".to_string(),
            refresh_token: "".to_string(),
            id_token: "".to_string(),
            token_type: String::from("Bearer"),
            expires_in: 0,
        }
    }

    pub(crate) fn with_access_token(mut self, access_token: String) -> Self {
        self.access_token = access_token;
        self
    }
}