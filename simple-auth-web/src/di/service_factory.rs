use std::sync::Arc;
use simple_auth_crud::crypto::SecretStore;
use simple_auth_crud::DbContext;

pub trait DataFactory<T> {
    fn add_data(&mut self, data: T) -> &Self;
    fn get_data(&self) -> Option<&T>;
}

trait SingletonFactory<T> {
    fn add_singleton(self, singleton: T) -> Self;
    fn get_singleton(&self) -> Option<Arc<T>>;
}

pub trait TransientFactory<T> where T: for<'t> From<&'t Self> {
    fn get_transient(&self) -> T {
        T::from(self)
    }
}

pub struct ServiceFactory<'r> {
    db_context: Option<Arc<DbContext<'r>>>,
    secret_store: Option<Arc<SecretStore>>,
}

impl ServiceFactory<'_> {
    pub fn new() -> Self {
        Self {
            db_context: None,
            secret_store: None,
        }
    }

    pub fn add_singleton<T>(mut self, singleton: T) -> Self
        where Self: SingletonFactory<T>
    {
        <Self as SingletonFactory<T>>::add_singleton(self, singleton)
    }

    pub fn get_singleton<T>(&self) -> Option<Arc<T>>
        where Self: SingletonFactory<T>
    {
        <Self as SingletonFactory<T>>::get_singleton(self)
    }
}

impl <T>TransientFactory<T> for ServiceFactory<'_> where T: for<'t> From<&'t Self> {

}

impl <'r>SingletonFactory<DbContext<'r>> for ServiceFactory<'r> {
    fn add_singleton(mut self, singleton: DbContext<'r>) -> Self {
        self.db_context = Some(Arc::new(singleton));
        self
    }

    fn get_singleton(&self) -> Option<Arc<DbContext<'r>>> {
        self.db_context.clone()
    }
}

impl SingletonFactory<SecretStore> for ServiceFactory<'_> {
    fn add_singleton(mut self, singleton: SecretStore) -> Self {
        self.secret_store = Some(Arc::new(singleton));
        self
    }

    fn get_singleton(&self) -> Option<Arc<SecretStore>> {
        self.secret_store.clone()
    }
}