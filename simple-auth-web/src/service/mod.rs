use crate::error::ServiceError;

mod realm_service;
mod role_service;
mod user_service;
mod auth_service;

pub type RealmService<'r> = realm_service::RealmService<'r>;
pub type RoleService<'r> = role_service::RoleService<'r>;
pub type UserService<'r> = user_service::UserService<'r>;
pub type AuthService<'r> = auth_service::AuthService<'r>;

pub(crate) trait Service<T> {
    async fn get_all(&self) -> Result<Vec<T>, ServiceError>;
}