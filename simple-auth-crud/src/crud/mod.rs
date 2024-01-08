mod realm_crud;
mod abs;
mod role_crud;

pub type RealmCrud<'r> = realm_crud::RealmCrud<'r>;
pub type RoleCrud<'r> = role_crud::RoleCrud<'r>;