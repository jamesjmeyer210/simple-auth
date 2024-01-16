use std::sync::Arc;
use simple_auth_model::Role;
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

        if model.realms.len() == 0 {
            log::warn!("No realms associated with {}", &model.name);
            return Ok(model);
        }

        let realms: Vec<&String> = model.realms.iter().map(|x|&x.name).collect();
        let c = self._roles_to_realms.add_realms_to_role(&model.name, &realms).await?;
        log::debug!("Added {} realms to {}", c, &model.name);

        Ok(model)
    }

    pub async fn get_all(&self) -> Result<Vec<Role>,sqlx::Error> {
        Ok(self._roles.all()
            .await?
            .drain(0..)
            .map(|x|x.into())
            .collect())
    }
}