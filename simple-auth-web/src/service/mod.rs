mod realm_service;
mod role_service;
mod user_service;

pub type RealmService<'r> = realm_service::RealmService<'r>;
pub type RoleService<'r> = role_service::RoleService<'r>;
pub type UserService<'r> = user_service::UserService<'r>;