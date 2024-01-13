use simple_auth_crud::crud::RealmCrud;
use simple_auth_crud::DbContext;
use simple_auth_model::Realm;
use crate::di::{ServiceFactory};
use crate::error::ServiceError;

pub struct RealmService<'r> {
    _crud: RealmCrud<'r>
}

impl <'r>From<&ServiceFactory<'_>> for RealmService<'r> {
    fn from(value: &ServiceFactory) -> Self {
        Self {
            _crud: value.get_singleton::<DbContext>().map(|x|x.as_ref()).unwrap().into()
        }
    }
}

impl <'r>RealmService<'r> {
    pub async fn add_default(&self) -> Result<Realm,ServiceError> {
        let realm = Realm::default();

        let exists = self._crud.contains(&realm.name)
            .await
            .map_err(|e|ServiceError::from(e))?;

        if exists {
            log::debug!("Default realm {} exists", &realm.name);
            return Ok(realm);
        }

        let realm = self._crud.add(&realm.name)
            .await
            .map_err(|e|ServiceError::from(e))?;

        log::debug!("Added default realm {}", &realm.name);
        Ok(realm)
    }

    pub async fn get_all(&self) -> Result<Vec<Realm>,ServiceError> {
        self._crud.get_all()
            .await
            .map_err(|e|ServiceError::from(e))
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Realm,ServiceError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use simple_auth_crud::crud::RealmCrud;
    use simple_auth_crud::DbContext;
    use crate::di::{ServiceFactory, SingletonFactory, TransientFactory};
    use crate::service::RealmService;

    #[actix_rt::test]
    async fn from_compiles(){
        let db = DbContext::in_memory().await.unwrap();

        let factory = ServiceFactory::new()
            .add_singleton(db);

        let service = factory.get_transient::<RealmService>();
    }
}