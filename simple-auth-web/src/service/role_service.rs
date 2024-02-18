use std::sync::Arc;
use simple_auth_crud::crud::{RealmCrud, RoleCrud};
use simple_auth_crud::DbContext;
use simple_auth_model::{Realm, Role};
use simple_auth_model::role::RoleUpdate;
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
        log::debug!("Retrieving all roles");
        crud.get_all()
            .await
            .map_err(|e|ServiceError::from(e))
    }
}

impl <'r>RoleService<'r> {
    pub async fn get_by_id(&self, id: &str) -> Result<Role,ServiceError> {
        let crud = self.db_context.get_crud::<RoleCrud>();
        log::debug!("Retrieving role \"{}\"", id);
        crud.get_by_id(id)
            .await
            .map_err(|e|ServiceError::from(e))
    }

    pub async fn add_default(&self, realm: String) -> Result<Role,ServiceError> {
        log::debug!("Adding default role to realm \"{}\"", &realm);

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

    pub async fn add(&self, name: String, max: Option<u32>, realm: String) -> Result<Role,ServiceError> {
        let realm_crud = self.db_context.get_crud::<RealmCrud>();
        log::debug!("Adding role \"{}\"", &name);

        let realm = realm_crud.get_by_id(&realm)
            .await
            .map_err(|e|ServiceError::from(e))?;

        let role = Role::new(name, max, &realm);

        let role_crud = self.db_context.get_crud::<RoleCrud>();
        role_crud.add(role)
            .await
            .map_err(|e|ServiceError::from(e))
    }

    pub async fn update(&self, update: RoleUpdate) -> Result<String,ServiceError> {
        let role_curd = self.db_context.get_crud::<RoleCrud>();
        role_curd.update(update)
            .await
            .map_err(|e|ServiceError::from(e))
    }

    pub async fn soft_delete_by_id(&self, id: &str) -> Result<(),ServiceError> {
        let role_curd = self.db_context.get_crud::<RoleCrud>();
        role_curd.soft_delete_by_id(id)
            .await
            .map_err(|e|ServiceError::from(e))
    }
}