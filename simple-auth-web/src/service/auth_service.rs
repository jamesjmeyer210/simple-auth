use std::sync::Arc;
use simple_auth_crud::crud::UserCrud;
use simple_auth_crud::crypto::SecretStore;
use simple_auth_crud::DbContext;
use simple_auth_model::jwt::Jwt;
use simple_auth_model::Password;
use crate::error::ServiceError;

pub struct AuthService<'r> {
    db_context: Arc<DbContext<'r>>,
    secret_store: Arc<SecretStore>,
}

impl <'r>AuthService<'r> {
    pub async fn get_jwt(&self, user_name: String, password: Password) -> Result<Jwt,ServiceError> {
        let crud = self.db_context.get_crud::<UserCrud>();
        let user = crud.get_by_name(&user_name)
            .await
            .map_err(|e|ServiceError::from(e))?;

        let valid_pass = match &user.password {
            None => false,
            Some(user_pass) => user_pass == &password
        };
        if !valid_pass {
            return Err(ServiceError::InvalidArgument);
        }

        todo!()
    }
}