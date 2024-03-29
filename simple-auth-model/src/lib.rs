mod password;
mod email;
mod ip_address;
mod contact_info;
mod error;
mod limit_vec;

// public modules
pub mod abs;
pub mod user;
pub mod encoding;
pub mod config;
pub mod auth;
pub mod realm;
pub mod role;

// re-exports
pub use chrono;
pub use uuid;
pub use log4rs;

pub type Realm = realm::Realm;
pub type Role = role::Role;
pub type User = user::User;
pub type Password = password::Password;
pub type Email = email::Email;
pub type ContactInfo = contact_info::ContactInfo;
pub type ContactValue = contact_info::ContactValue;
pub type LimitVec<T> = limit_vec::LimitVec<T>;