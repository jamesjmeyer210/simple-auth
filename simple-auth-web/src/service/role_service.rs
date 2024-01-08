use simple_auth_crud::crud::RoleCrud;
use simple_auth_crud::DbContext;
use simple_auth_model::{Realm, Role};
use crate::di::ServiceProvider;
use crate::error::ServiceError;

pub struct RoleService {
    _crud: RoleCrud<'static>
}

impl From<&ServiceProvider> for RoleService {
    fn from(value: &ServiceProvider) -> Self {
        Self {
            _crud: value.get::<DbContext>().unwrap().into()
        }
    }
}

impl RoleService {
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