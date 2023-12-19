
pub trait Crud<T> {
    type Id;

    fn add(&self, model: &T) -> T;
    fn get_by_id(&self, id: &Self::Id) -> Option<T>;
    fn update(&self, model: &T) -> T;
    fn delete(&self, id: &Self::Id) -> T;
}