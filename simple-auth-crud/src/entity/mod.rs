mod user_entity;
mod password_hash;
mod contact_info_entity;
mod realm_entity;
mod role_entity;

type PasswordHash = password_hash::PasswordHash;
type ContactInfoEntity = contact_info_entity::ContactInfoEntity;
type UserEntity = user_entity::UserEntity;
type RealmEntity = realm_entity::Realm;
type RoleEntity = role_entity::RoleEntity;