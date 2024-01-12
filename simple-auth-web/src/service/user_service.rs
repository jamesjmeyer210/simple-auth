use std::sync::Arc;
use simple_auth_crud::crud::UserCrud;
use simple_auth_crud::crypto::SecretStore;
use simple_auth_crud::DbContext;
use simple_auth_model::{ContactInfo, Realm, Role, User};
use crate::di::ServiceProvider;
use crate::error::ServiceError;

pub struct UserService {
    _crud: UserCrud<'static>,
    _secret_store: Arc<SecretStore>,
}

impl From<&ServiceProvider> for UserService {
    fn from(value: &ServiceProvider) -> Self {
        Self {
            _crud: value.get::<DbContext>().unwrap().into(),
            _secret_store: value.get::<Arc<SecretStore>>().unwrap().clone(),
        }
    }
}

impl UserService {
    pub async fn add_default(&self, realm: Realm, role: Role) -> Result<User,ServiceError> {
        let user = User::default()
            .with_realm(realm)
            .with_role(role)
            .with_contact_info(ContactInfo::default());

        if self._crud.contains_by_name(&user.name).await? {
            log::debug!("Default user {} exists", &user.name);
            return Ok(user);
        }

        self._crud.add(&user, self._secret_store.as_ref()).await?;
        Ok(user)
    }
}