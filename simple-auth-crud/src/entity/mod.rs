mod user_entity;
mod password_hash;
mod contact_info_entity;
mod realm_entity;
mod role_entity;

pub(crate) type PasswordHash = password_hash::PasswordHash;
pub(crate) type ContactInfoEntity = contact_info_entity::ContactInfoEntity;
pub(crate) type UserEntity = user_entity::UserEntity;
pub(crate) type RealmEntity = realm_entity::Realm;
pub(crate) type RoleEntity = role_entity::RoleEntity;