use std::sync::Arc;
use simple_auth_crud::crud::UserCrud;
use simple_auth_crud::crypto::SecretStore;
use simple_auth_crud::DbContext;
use simple_auth_model::{ContactInfo, Realm, Role, User};
use crate::di::{ServiceFactory};
use crate::error::ServiceError;

pub struct UserService<'r> {
    db_context: Arc<DbContext<'r>>,
    secret_store: Arc<SecretStore>,
}

impl <'r>From<&ServiceFactory<'r>> for UserService<'r> {
    fn from(value: &ServiceFactory<'r>) -> Self {
        Self {
            db_context: value.get_singleton::<DbContext>().unwrap(),
            secret_store: value.get_singleton::<SecretStore>().unwrap(),
        }
    }
}

impl <'r>UserService<'r> {
    pub async fn add_default(&self, realm: Realm, role: Role) -> Result<User,ServiceError> {
        let user = User::default()
            .with_realm(realm)
            .with_role(role)
            .with_contact_info(ContactInfo::default());

        let crud = self.db_context.get_crud::<UserCrud>();
        if crud.contains_by_name(&user.name).await? {
            log::debug!("Default user {} exists", &user.name);
            return Ok(user);
        }

        crud.add(&user, self.secret_store.as_ref()).await?;
        Ok(user)
    }
}