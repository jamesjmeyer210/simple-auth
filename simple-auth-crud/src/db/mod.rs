mod db_context;
mod realm_table;
mod user_table;
mod role_table;
mod roles_to_realms_table;
mod users_to_realms_table;
mod users_to_roles_table;
mod user_contact_info_table;
mod secret_table;

pub(crate) type DbContext<'r> = db_context::DbContext<'r>;