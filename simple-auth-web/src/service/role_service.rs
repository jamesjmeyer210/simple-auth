use std::sync::Arc;
use simple_auth_crud::crud::RoleCrud;
use simple_auth_crud::DbContext;
use simple_auth_model::{Realm, Role};
use crate::di::{ServiceFactory};
use crate::error::ServiceError;

pub struct RoleService<'r> {
    db_context: Arc<DbContext<'r>>
}

impl <'r>From<&ServiceFactory<'r>> for RoleService<'r> {
    fn from(value: &ServiceFactory<'r>) -> Self {
        Self {
            db_context: value.get_singleton::<DbContext>().unwrap(),
        }
    }
}

impl <'r>RoleService<'r> {
    pub async fn add_default(&self, realm: Realm) -> Result<Role,ServiceError> {
        let role = Role::default().with_realm(realm);

        let crud = self.db_context.get_crud::<RoleCrud>();

        if crud.contains(&role.name).await? {
            log::debug!("Default role {} exists", &role.name);
            return Ok(role);
        }

        let role = crud.add(role)
            .await
            .map_err(|e|ServiceError::from(e))?;

        log::debug!("Added role {}", &role.name);
        Ok(role)
    }
}