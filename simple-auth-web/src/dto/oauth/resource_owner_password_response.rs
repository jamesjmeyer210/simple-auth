use serde::Serialize;
use simple_auth_model::auth::ResourceOwnerTokens;

#[derive(Debug, Serialize)]
pub(crate) struct ResourceOwnerPasswordResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl ResourceOwnerPasswordResponse {
    pub(crate) fn bearer() -> Self {
        Self {
            access_token: "".to_string(),
            refresh_token: "".to_string(),
            id_token: "".to_string(),
            token_type: String::from("bearer"),
            expires_in: 0,
        }
    }

    pub(crate) fn with_resource_owner_tokens(mut self, tokens: ResourceOwnerTokens) -> Self {
        self.access_token = tokens.access_token;
        self.refresh_token = tokens.refresh_token;
        self.expires_in = tokens.expires_in;
        self
    }
}