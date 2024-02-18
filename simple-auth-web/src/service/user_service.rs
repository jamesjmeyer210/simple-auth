use std::sync::Arc;
use simple_auth_crud::crud::UserCrud;
use simple_auth_crud::crypto::SecretStore;
use simple_auth_crud::DbContext;
use simple_auth_model::{ContactInfo, LimitVec, Realm, Role, User};
use simple_auth_model::uuid::Uuid;
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

    pub async fn add(&self, user: User) -> Result<User,ServiceError> {
        let crud = self.db_context.get_crud::<UserCrud>();
        crud.add(&user, self.secret_store.as_ref()).await?;
        Ok(user)
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<User,ServiceError> {
        let crud = self.db_context.get_crud::<UserCrud>();
        crud.get_by_id(id)
            .await
            .map_err(ServiceError::from)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<User,ServiceError> {
        let crud = self.db_context.get_crud::<UserCrud>();
        crud.get_by_name(name)
            .await
            .map_err(ServiceError::from)
    }

    pub async fn get_by_contact(&self, contact: &str) -> Result<User,ServiceError> {
        let crud = self.db_context.get_crud::<UserCrud>();
        crud.get_by_contact(contact)
            .await
            .map_err(ServiceError::from)
    }

    pub async fn get_page(&self, page: u32) -> Result<LimitVec<User>,ServiceError> {
        let crud = self.db_context.get_crud::<UserCrud>();
        crud.get_limited_range(100, page * 100)
            .await
            .map_err(ServiceError::from)
    }
}