use sqlx::{query_as, QueryBuilder, Sqlite};
use simple_auth_model::uuid::Uuid;
use crate::abs::join_table::JoinTable;
use crate::entity::{RealmEntity, RoleEntity, UserEntity};

impl <'r>JoinTable<'r, UserEntity, RoleEntity> {
    pub(crate) async fn add_roles_to_user(&self, user_id: &Uuid, realms: &Vec<&String>)
                                           -> Result<u64,sqlx::Error>
    {
        let relations = realms.iter().map(|realm|(user_id, realm));

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO `users_to_roles` (`user_id`, `role_id`)"
        );

        query_builder.push_values(relations.take(realms.len()), |mut b, relation|{
            b.push_bind(relation.0)
                .push_bind(relation.1);
        });

        let query = query_builder.build();
        query.execute(&*self.pool).await.map(|x|x.rows_affected())
    }

    pub(crate) async fn get_roles_by_user_id(&self, user_id: &Uuid) -> Result<Vec<RoleEntity>,sqlx::Error> {
        query_as(
            r#"
            select `name`, `max`, `created_on`, `deleted_on`
            from `roles` as `a`
            where `a`.`name` = (
                select `role_id`
                from `users_to_roles` as `b`
                where `b`.`user_id` = ?
            )
            "#)
            .bind(user_id)
            .fetch_all(&*self.pool)
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::db::DbContext;

    #[sqlx::test]
    async fn get_roles_by_user_id_returns_roles() {
        let db = DbContext::in_memory().await.unwrap();
        let user = db.init_default_unchecked().await;

        let roles = db.users_to_roles.get_roles_by_user_id(&user.id).await;
        assert!(roles.is_ok());

        let roles = roles.unwrap();
        assert_eq!(1, roles.len());
    }
}