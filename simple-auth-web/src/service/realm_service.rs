use std::sync::Arc;
use simple_auth_crud::crud::RealmCrud;
use simple_auth_crud::DbContext;
use simple_auth_model::Realm;
use simple_auth_model::realm::UpdateRealm;
use crate::di::{ServiceFactory};
use crate::error::ServiceError;
use crate::service::Service;

pub struct RealmService<'r> {
    db_context: Arc<DbContext<'r>>,
}

impl <'r>From<&ServiceFactory<'r>> for RealmService<'r> {
    fn from(value: &ServiceFactory<'r>) -> Self {
        Self {
            db_context: value.get_singleton::<DbContext>().unwrap()
        }
    }
}

impl <'r>Service<Realm> for RealmService<'r> {
    async fn get_all(&self) -> Result<Vec<Realm>, ServiceError> {
        let crud = self.db_context.get_crud::<RealmCrud>();

        crud.get_all()
            .await
            .map_err(|e|ServiceError::from(e))
    }
}

impl <'r>RealmService<'r> {
    pub async fn add_default(&self) -> Result<Realm,ServiceError> {
        let realm = Realm::default();

        let crud = self.db_context.get_crud::<RealmCrud>();

        let exists = crud.contains(&realm.name)
            .await
            .map_err(|e|ServiceError::from(e))?;

        if exists {
            log::debug!("Default realm {} exists", &realm.name);
            return Ok(realm);
        }

        let realm = crud.add(&realm.name)
            .await
            .map_err(|e|ServiceError::from(e))?;

        log::debug!("Added default realm {}", &realm.name);
        Ok(realm)
    }

    pub async fn add(&self, realm: &str) -> Result<Realm,ServiceError> {
        let crud = self.db_context.get_crud::<RealmCrud>();
        crud.add(realm)
            .await
            .map_err(|e|ServiceError::from(e))
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Realm,ServiceError> {
        let crud = self.db_context.get_crud::<RealmCrud>();
        crud.get_by_id(id)
            .await
            .map_err(|e|ServiceError::from(e))
    }

    pub async fn update(&self, update: UpdateRealm) -> Result<String, ServiceError> {
        let crud = self.db_context.get_crud::<RealmCrud>();
        crud.update(update)
            .await
            .map_err(|e|ServiceError::from(e))
    }

    pub async fn soft_delete_by_id(&self, id: &str) -> Result<(), ServiceError> {
        let crud = self.db_context.get_crud::<RealmCrud>();
        crud.soft_delete_by_id(id)
            .await
            .map_err(|e|ServiceError::from(e))
    }
}

#[cfg(test)]
mod test {
    use simple_auth_crud::crud::RealmCrud;
    use simple_auth_crud::DbContext;
    use crate::di::{ServiceFactory, TransientFactory};
    use crate::service::RealmService;

    #[actix_rt::test]
    async fn from_compiles(){
        let db = DbContext::in_memory().await.unwrap();

        let factory = ServiceFactory::new()
            .add_singleton(db);

        let service: RealmService = factory.get_transient();
    }
}