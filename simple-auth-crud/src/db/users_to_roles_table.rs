use sqlx::{QueryBuilder, Sqlite};
use simple_auth_model::uuid::Uuid;
use crate::abs::join_table::JoinTable;
use crate::entity::{RoleEntity, UserEntity};

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
}