use simple_auth_crud::crud::RealmCrud;
use simple_auth_crud::DbContext;
use simple_auth_model::Realm;
use crate::di::ServiceProvider;
use crate::error::ServiceError;

pub struct RealmService {
    _crud: RealmCrud<'static>
}

impl <'a>From<&'a ServiceProvider> for RealmService {
    fn from(value: &'a ServiceProvider) -> Self {
        let db = value.get::<DbContext>().unwrap();
        Self {
            _crud: db.get_crud::<RealmCrud>()
        }
    }
}

impl RealmService {
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
}

#[cfg(test)]
mod test {
    use simple_auth_crud::crud::RealmCrud;
    use simple_auth_crud::DbContext;
    use crate::di::ServiceCollection;
    use crate::service::RealmService;

    #[actix_rt::test]
    async fn from_compiles(){
        let db = DbContext::in_memory().await.unwrap();

        let mut services = ServiceCollection::new();
        services.add(db);

        let provider = services.build_provider();
        let service = provider.get_transient::<RealmService>();
    }
}