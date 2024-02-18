use std::sync::Arc;
use simple_auth_model::Realm;
use simple_auth_model::realm::UpdateRealm;
use crate::abs::table::Table;
use crate::db::DbContext;
use crate::entity::RealmEntity;

pub struct RealmCrud<'r> {
    _realms: Arc<Table<'r, RealmEntity>>,
}

impl <'r>From<&DbContext<'r>> for RealmCrud<'r> {
    fn from(value: &DbContext<'r>) -> Self {
        Self {
            _realms: value.realms.clone(),
        }
    }
}

impl <'r>RealmCrud<'r> {
    pub async fn add(&self, realm: &str) -> Result<Realm,sqlx::Error> {
        let entity = RealmEntity::from(realm);
        let _ = self._realms.add(&entity).await?;
        Ok(entity.into())
    }

    pub async fn get_all(&self) -> Result<Vec<Realm>, sqlx::Error> {
        self._realms.all()
            .await?
            .drain(0..)
            .map(|x|Ok(x.into()))
            .collect()
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Realm, sqlx::Error> {
        self._realms.get_by_id(id)
            .await
            .map(|x|x.into())
    }

    pub async fn get_by_names(&self, names: &Vec<String>) -> Result<Vec<Realm>, sqlx::Error> {
        self._realms.get_by_names(names)
            .await?
            .drain(0..)
            .map(|x|Ok(x.into()))
            .collect()
    }

    pub async fn contains(&self, realm: &str) -> Result<bool,sqlx::Error> {
        self._realms.count_by_name(realm)
            .await
            .map(|x|x == 1)
    }

    pub async fn update(&self, update: UpdateRealm) -> Result<String, sqlx::Error> {
        let c = self._realms.update(&update).await?;
        log::debug!("{} realms updated", c);
        Ok(update.rename)
    }

    pub async fn soft_delete_by_id(&self, id: &str) -> Result<(), sqlx::Error> {
        let c = self._realms.soft_delete_by_id(id).await?;
        log::debug!("Soft-deleted {}", id);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::db::DbContext;
    use super::RealmCrud;

    #[sqlx::test]
    async fn add_returns_ok(){
        let db = DbContext::in_memory().await.unwrap();

        let crud = RealmCrud::from(&db);
        let realm = crud.add("master").await;
        assert!(realm.is_ok());

        let contains = crud.contains("master").await;
        assert!(contains.is_ok());
        let contains = contains.unwrap();
        assert!(contains)
    }
}