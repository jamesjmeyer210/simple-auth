use simple_auth_crud::crud::RealmCrud;
use simple_auth_crud::DbContext;
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
    pub(crate) async fn add_default(&self) -> Result<(),ServiceError> {
        let realm = "master";

        let exists = self._crud.contains(realm)
            .await
            .map_err(|e|ServiceError::from(e))?;

        if exists {
            log::debug!("{} exists", realm);
            return Ok(());
        }

        let x = self._crud.add(realm)
            .await
            .map_err(|e|ServiceError::from(e))?;

        log::debug!("added {} realm", &x.name);
        Ok(())
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