use std::sync::Arc;
use aes_gcm::Aes256Gcm;
use simple_auth_model::abs::AsBytes;
use simple_auth_model::{ContactInfo, ContactValue, LimitVec, Password, User};
use simple_auth_model::user::{FullUser, PartialUser};
use simple_auth_model::uuid::Uuid;
use crate::abs::join_table::JoinTable;
use crate::abs::table::Table;
use crate::crypto;
use crate::crypto::{Encrypted, Secret, SecretStore, Sha256Hash};
use crate::db::DbContext;
use crate::entity::{ContactInfoEntity, RealmEntity, RoleEntity, UserEntity};

pub struct UserCrud<'r> {
    users: Arc<Table<'r, UserEntity>>,
    contacts: Arc<Table<'r, ContactInfoEntity>>,
    realms: Arc<JoinTable<'r, UserEntity, RealmEntity>>,
    roles: Arc<JoinTable<'r, UserEntity, RoleEntity>>,
}

impl <'r>From<&DbContext<'r>> for UserCrud<'r> {
    fn from(value: &DbContext<'r>) -> Self {
        Self {
            users: value.users.clone(),
            contacts: value.user_contacts.clone(),
            realms: value.users_to_realms.clone(),
            roles: value.users_to_roles.clone(),
        }
    }
}

impl <'r>UserCrud<'r> {
    pub async fn add<'u>(&self, user: &'u User, secret_store: &SecretStore) -> Result<&'u User,sqlx::Error> {
        let realms: Vec<&String> = user.realms.iter().map(|x|&x.name).collect();
        let roles: Vec<&String> = user.roles.iter().map(|x|&x.name).collect();
        let entity = UserEntity::from(user);

        let c = self.users.add(&entity).await?;
        log::debug!("Added {} user", c);

        if realms.len() > 0 {
            let c = self.realms.add_realms_to_user(&entity.id, &realms).await?;
            log::debug!("Added {} realms to user {}", c, &entity.name);
        }

        if roles.len() > 0 {
            let c = self.roles.add_roles_to_user(&entity.id, &roles).await?;
            log::debug!("Added {} roles to user {}", c, &entity.name);
        }

        if user.contact_info.len() == 0 {
            return Ok(user);
        }

        // TODO: Contact Info should be encrypted by the user password - not a secret
        let contacts = user.contact_info.iter().map(|x|{
            let mut entity = ContactInfoEntity::new(x, &user.id);
            entity.hash = Sha256Hash::from(x.value.as_bytes()).into();
            //entity.enc = secret_store.encrypt::<Aes256Gcm>(x.value.as_bytes()).unwrap().into();
            entity.enc = crypto::encrypt::<Aes256Gcm>(
                x.value.as_bytes(),
                user.password.as_ref().unwrap().as_bytes())
                .unwrap()
                .into();
            entity
        }).collect();
        let c = self.contacts.add_contacts(&contacts).await?;
        log::debug!("Added {} contacts to user {}", c, &entity.name);

        Ok(user)
    }

    pub async fn contains_by_name(&self, user_name: &str) -> Result<bool,sqlx::Error> {
        self.users.count_by_name(user_name)
            .await
            .map(|x|x == 1)
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<User,sqlx::Error> {
        self.users.get_by_id(id)
            .await
            .map(|x|x.into())
    }

    pub async fn get_by_name(&self, id: &str) -> Result<User,sqlx::Error> {
        self.users.get_by_name(id)
            .await
            .map(|x|x.into())
    }

    pub async fn get_full_by_name(&self, name: &str, password: Password) -> Result<FullUser,sqlx::Error> {
        let user: PartialUser = self.users.get_by_name(name)
            .await
            .map(|x|x.into())?;

        let secret = Secret::try_from(password.as_bytes().to_vec()).unwrap();

        // TODO: complex mappings such as these ought to go somewhere else
        let contact_info: Vec<ContactInfo> = self.contacts.get_by_user_id(&user.id)
            .await?
            .drain(0..)
            .map(|x|{
                let enc = Encrypted::<Aes256Gcm>::try_from(x.enc).unwrap();
                let raw: Vec<u8> = enc.decrypt(&secret.as_bytes()).unwrap();
                // TODO: this mapping is incomplete
                ContactInfo {
                    verified: x.verified,
                    label: x.label,
                    value: ContactValue::Other(String::from_utf8(raw).unwrap())
                }
            })
            .collect();

        // TODO: break this apart into smaller functions
        let realms: Vec<String> = self.realms.get_realms_by_user_id(&user.id)
            .await?
            .drain(0..)
            .map(|x|x.name)
            .collect();
        // TODO: break this apart into smaller functions
        let roles: Vec<String> = self.roles.get_roles_by_user_id(&user.id)
            .await?
            .drain(0..)
            .map(|x|x.name)
            .collect();
        Ok(FullUser::new(user, contact_info, realms, roles))
    }

    pub async fn get_by_contact(&self, contact: &str) -> Result<User,sqlx::Error> {
        let hash = Sha256Hash::from(contact.as_bytes());
        let id = self.contacts.get_user_id_by_hash(&hash).await?;
        self.users.get_by_id(&id)
            .await
            .map(|x|x.into())
    }

    pub async fn get_limited_range(&self, limit: u32, offset: u32) -> Result<LimitVec<User>, sqlx::Error> {
        let total = self.users.count().await?;
        let users = self.users.get_range(limit, offset)
            .await?
            .drain(0..)
            .map(|x|x.into())
            .collect();

        Ok(LimitVec {
            total,
            data: users,
        })
    }
}

#[cfg(test)]
mod test {
    use simple_auth_model::Password;
    use crate::crud::UserCrud;
    use crate::DbContext;

    #[sqlx::test]
    async  fn get_full_by_name_returns_user() {
        let db = DbContext::in_memory().await.unwrap();
        let user = db.init_default_unchecked().await;

        let crud = db.get_crud::<UserCrud>();
        let full_user = crud.get_full_by_name(
            "root",
            Password::try_from("password123").unwrap()
        ).await;

        assert!(full_user.is_ok());
        let full_user = full_user.unwrap();

        assert_eq!(1, full_user.realms.len());
        assert_eq!(1, full_user.roles.len());
    }
}