use std::sync::Arc;
use aes_gcm::Aes256Gcm;
use simple_auth_model::abs::AsBytes;
use simple_auth_model::User;
use crate::abs::join_table::JoinTable;
use crate::abs::table::Table;
use crate::crypto::{SecretStore, Sha256Hash};
use crate::db::DbContext;
use crate::entity::{ContactInfoEntity, RealmEntity, RoleEntity, UserEntity};

pub struct UserCrud<'r> {
    _users: Arc<Table<'r, UserEntity>>,
    _contacts: Arc<Table<'r, ContactInfoEntity>>,
    _realms: Arc<JoinTable<'r, UserEntity, RealmEntity>>,
    _roles: Arc<JoinTable<'r, UserEntity, RoleEntity>>,
}

impl <'r>From<&DbContext<'r>> for UserCrud<'r> {
    fn from(value: &DbContext<'r>) -> Self {
        Self {
            _users: value.users.clone(),
            _contacts: value.user_contacts.clone(),
            _realms: value.users_to_realms.clone(),
            _roles: value.users_to_roles.clone(),
        }
    }
}

impl <'r>UserCrud<'r> {
    pub async fn contains_by_name(&self, user_name: &str) -> Result<bool,sqlx::Error> {
        self._users.count_by_name(user_name)
            .await
            .map(|x|x == 1)
    }

    pub async fn add<'u>(&self, user: &'u User, secret_store: &SecretStore) -> Result<&'u User,sqlx::Error> {
        let realms: Vec<&String> = user.realms.iter().map(|x|&x.name).collect();
        let roles: Vec<&String> = user.roles.iter().map(|x|&x.name).collect();
        let entity = UserEntity::from(user);

        let c = self._users.add(&entity).await?;
        log::debug!("Added {} user", c);

        if realms.len() > 0 {
            let c = self._realms.add_realms_to_user(&entity.id, &realms).await?;
            log::debug!("Added {} realms to user {}", c, &entity.name);
        }

        if roles.len() > 0 {
            let c = self._roles.add_roles_to_user(&entity.id, &roles).await?;
            log::debug!("Added {} roles to user {}", c, &entity.name);
        }

        if user.contact_info.len() == 0 {
            return Ok(user);
        }

        let contacts = user.contact_info.iter().map(|x|{
            let mut entity = ContactInfoEntity::new(x, &user.id);
            entity.hash = Sha256Hash::from(x.value.as_bytes()).into();
            entity.enc = secret_store.encrypt::<Aes256Gcm>(x.value.as_bytes()).unwrap().into();
            entity
        }).collect();
        let c = self._contacts.add_contacts(&contacts).await?;
        log::debug!("Added {} contacts to user {}", c, &entity.name);

        Ok(user)
    }
}