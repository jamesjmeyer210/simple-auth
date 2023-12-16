use simple_auth_model::chrono::{DateTime, Utc};

pub(crate) trait Entity<T> {
    fn primary_key(&self) -> &T;
    fn created_on(&self) -> &DateTime<Utc>;
}