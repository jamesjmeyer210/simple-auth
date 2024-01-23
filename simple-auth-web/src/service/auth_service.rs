use std::sync::Arc;
use simple_auth_crud::crud::UserCrud;
use simple_auth_crud::crypto::SecretStore;
use simple_auth_crud::DbContext;
use simple_auth_model::abs::AsJson;
use simple_auth_model::chrono::Utc;
use simple_auth_model::jwt::{JwtClaims, Jwt, JwtHeader};
use simple_auth_model::Password;
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
    pub async fn get_jwt(&self, user_name: String, password: Password) -> Result<Jwt,ServiceError> {
        let crud = self.db_context.get_crud::<UserCrud>();
        let user = crud.get_full_by_name(&user_name, password).await?;

        let claims = JwtClaims {
            name: user.name,
            roles: user.roles,
            realms: user.realms,
            auth_time: Utc::now(),
        };
        let header = JwtHeader::default();
        let signature = self.secret_store.sign_jwt(&header.as_json().unwrap(), &claims.as_json().unwrap());

        Ok(Jwt {
            header,
            claims,
            signature,
        })
    }
}