use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct ServiceCollection {
    _data: HashMap<TypeId, Box<dyn Any>>,
}

impl ServiceCollection {
    pub fn new() -> Self {
        Self {
            _data: HashMap::default(),
        }
    }

    pub fn add<T: 'static>(&mut self, value: T) {
        self._data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn build_provider(self) -> ServiceProvider {
        self.into()
    }
}

pub struct ServiceProvider {
    _data: HashMap<TypeId, Box<dyn Any>>,
}

impl From<ServiceCollection> for ServiceProvider {
    fn from(value: ServiceCollection) -> Self {
        Self {
            _data: value._data,
        }
    }
}

impl ServiceProvider {
    pub fn contains<T: 'static>(&self) -> bool {
        self._data.contains_key(&TypeId::of::<T>())
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self._data.get(&TypeId::of::<T>())
            .and_then(|boxed|boxed.downcast_ref())
    }

    pub fn get_transient<T: 'static>(&self) -> T where T: for<'a> From<&'a ServiceProvider> {
        T::from(self)
    }

    pub fn len(&self) -> usize {
        self._data.len()
    }
}

#[cfg(test)]
mod test {
    use simple_auth_crud::crud::RealmCrud;
    use simple_auth_crud::DbContext;
    use crate::di::ServiceCollection;
    use crate::service::RealmService;

    #[actix_rt::test]
    async fn add_data_test(){
        let db = DbContext::in_memory().await.unwrap();

        let mut services = ServiceCollection::new();
        services.add(db);

        let provider = services.build_provider();
        assert!(provider.contains::<DbContext>());

        let db = provider.get::<DbContext>();

        assert!(db.is_some())
    }
}