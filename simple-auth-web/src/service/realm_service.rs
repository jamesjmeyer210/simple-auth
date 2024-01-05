use simple_auth_crud::crud::RealmCrud;
use simple_auth_crud::DbContext;
use crate::di::ServiceProvider;

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