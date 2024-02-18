use std::sync::Arc;
use simple_auth_crud::crud::UserCrud;
use simple_auth_crud::crypto::SecretStore;
use simple_auth_crud::DbContext;
use simple_auth_model::abs::AsJson;
use simple_auth_model::chrono::{DateTime, Utc};
use simple_auth_model::encoding::JwtStr;
use simple_auth_model::auth::{JwtClaims, Jwt, JwtHeader, RefreshToken, RefreshTokenBase64, ResourceOwnerTokens};
use simple_auth_model::Password;
use simple_auth_model::user::FullUser;
use simple_auth_model::uuid::Uuid;
use crate::di::ServiceFactory;
use crate::error::ServiceError;

pub struct AuthService<'r> {
    db_context: Arc<DbContext<'r>>,
    secret_store: Arc<SecretStore>,
}

impl <'r>From<&ServiceFactory<'r>> for AuthService<'r> {
    fn from(value: &ServiceFactory<'r>) -> Self {
        Self {
            db_context: value.get_singleton::<DbContext>().unwrap(),
            secret_store: value.get_singleton::<SecretStore>().unwrap(),
        }
    }
}

impl <'r>AuthService<'r> {
    pub fn validate_jwt(&self, bearer_token: &str) -> bool {
        let jwt = JwtStr::try_from(bearer_token)
            .unwrap()
            .into_parts()
            .into();
        self.secret_store.validate_jwt(&jwt)
    }

    /// Retrieves the user and returns their details in the format of JWT claims
    pub async fn get_resource_owner_tokens(&self, user_name: String, password: Password) -> Result<ResourceOwnerTokens,ServiceError> {
        let crud = self.db_context.get_crud::<UserCrud>();
        let user = crud.get_full_by_name(&user_name, password).await?;

        let auth_time = Utc::now();
        let refresh_token = self.get_refresh_token(&user.id, &auth_time);
        let access_token = self.get_access_token(user, auth_time)?
            .to_base64_string();

        Ok(ResourceOwnerTokens {
            access_token,
            refresh_token,
            expires_in: 0,
        })
    }

    fn get_access_token(&self, user: FullUser, auth_time: DateTime<Utc>) -> Result<Jwt,ServiceError> {
        let claims = JwtClaims {
            name: user.name,
            user_id: user.id,
            roles: user.roles,
            realms: user.realms,
            auth_time,
        };
        let header = JwtHeader::default();
        let signature = self.secret_store.sign_jwt(&header.as_json().unwrap(), &claims.as_json().unwrap());

        Ok(Jwt {
            header,
            claims,
            signature,
        })
    }

    fn get_refresh_token(&self, user_id: &Uuid, issued_on: &DateTime<Utc>) -> String {
        let token = RefreshToken::new(user_id, issued_on);
        let token: RefreshTokenBase64 = self.secret_store.hash_refresh_token(&token).into();
        token.into_inner()
    }
}