use std::sync::Arc;
use simple_auth_crud::crud::{RealmCrud, RoleCrud};
use simple_auth_crud::DbContext;
use simple_auth_model::{Realm, Role};
use crate::di::{ServiceFactory};
use crate::error::ServiceError;
use crate::service::Service;

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

impl <'r>Service<Role> for RoleService<'r> {
    async fn get_all(&self) -> Result<Vec<Role>, ServiceError> {
        let crud = self.db_context.get_crud::<RoleCrud>();

        crud.get_all()
            .await
            .map_err(|e|ServiceError::from(e))
    }
}

impl <'r>RoleService<'r> {
    pub async fn get_by_id(&self, id: &str) -> Result<Role,ServiceError> {
        let crud = self.db_context.get_crud::<RoleCrud>();

        crud.get_by_id(id)
            .await
            .map_err(|e|ServiceError::from(e))
    }

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

    pub async fn add(&self, name: String, max: Option<u32>, realms: Vec<String>) -> Result<Role,ServiceError> {
        let realm_crud = self.db_context.get_crud::<RealmCrud>();
        let realms_found = realm_crud.get_by_names(&realms)
            .await
            .map_err(|e|ServiceError::from(e))?;

        if realms_found.len() != realms.len() {
            return Err(ServiceError::InvalidArgument);
        }

        let role = Role::new(name, max, realms_found);

        let role_crud = self.db_context.get_crud::<RoleCrud>();
        role_crud.add(role)
            .await
            .map_err(|e|ServiceError::from(e))
    }
}