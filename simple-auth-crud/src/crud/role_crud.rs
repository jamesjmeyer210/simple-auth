use std::sync::Arc;
use simple_auth_model::Role;
use simple_auth_model::role::RoleUpdate;
use crate::abs::join_table::JoinTable;
use crate::abs::table::Table;
use crate::db::DbContext;
use crate::entity::{RealmEntity, RoleEntity};

pub struct RoleCrud<'r> {
    _roles: Arc<Table<'r, RoleEntity>>,
    _roles_to_realms: Arc<JoinTable<'r, RoleEntity, RealmEntity>>
}

impl <'r>From<&DbContext<'r>> for RoleCrud<'r> {
    fn from(value: &DbContext<'r>) -> Self {
        Self {
            _roles: value.roles.clone(),
            _roles_to_realms: value.roles_to_realms.clone(),
        }
    }
}

impl <'r>RoleCrud<'r> {
    pub async fn contains(&self, role: &str) -> Result<bool,sqlx::Error> {
        self._roles.count_by_name(role)
            .await
            .map(|x|x == 1)
    }

    pub async fn add(&self, model: Role) -> Result<Role,sqlx::Error> {
        let entity = RoleEntity::from(&model);
        let c = self._roles.add(&entity).await?;
        log::debug!("Added {} role", c);
        Ok(model)
    }

    pub async fn get_all(&self) -> Result<Vec<Role>,sqlx::Error> {
        Ok(self._roles.all()
            .await?
            .drain(0..)
            .map(|x|x.into())
            .collect())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Role, sqlx::Error> {
        self._roles.get_by_id(id)
            .await
            .map(|x|x.into())
    }

    pub async fn update(&self, update: RoleUpdate) -> Result<String, sqlx::Error> {
        let c = self._roles.update(&update).await?;
        log::debug!("Updated {} roles", c);
        Ok(update.rename)
    }
}