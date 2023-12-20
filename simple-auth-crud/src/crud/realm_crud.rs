use std::sync::Arc;
use simple_auth_model::Realm;
use crate::abs::table::Table;
use crate::db::DbContext;
use crate::entity::RealmEntity;

pub struct RealmCrud<'r> {
    _realms: Arc<Table<'r, RealmEntity, String>>,
}

impl From<&DbContext> for RealmCrud {
    fn from(value: &DbContext) -> Self {
        Self {
            _realms: value.realms.clone(),
        }
    }
}

impl RealmCrud {
    async fn add(&self, realm: &str) -> Result<Realm,sqlx::Error> {
        let entity = RealmEntity::from(realm);
        let _ = self._realms.add(&entity).await?;
        Ok(entity.into())
    }

    async fn get_all(&self) -> Result<Vec<Realm>, sqlx::Error> {
        self._realms.all()
            .await?
            .drain(0..)
            .map(|x|x.into())
            .collect()
    }
}