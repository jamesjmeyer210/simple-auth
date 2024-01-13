use simple_auth_crud::crud::RoleCrud;
use simple_auth_crud::DbContext;
use simple_auth_model::{Realm, Role};
use crate::di::{ServiceFactory};
use crate::error::ServiceError;

pub struct RoleService<'r> {
    _crud: RoleCrud<'r>
}

impl <'r>From<&ServiceFactory<'_>> for RoleService<'r> {
    fn from(value: &ServiceFactory) -> Self {
        Self {
            _crud: value.get_singleton::<DbContext>().map(|x|x.as_ref()).unwrap().into()
        }
    }
}

impl <'r>RoleService<'r> {
    pub async fn add_default(&self, realm: Realm) -> Result<Role,ServiceError> {
        let role = Role::default().with_realm(realm);

        if self._crud.contains(&role.name).await? {
            log::debug!("Default role {} exists", &role.name);
            return Ok(role);
        }

        let role = self._crud.add(role)
            .await
            .map_err(|e|ServiceError::from(e))?;

        log::debug!("Added role {}", &role.name);
        Ok(role)
    }
}