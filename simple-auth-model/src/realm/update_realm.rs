use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateRealm {
    pub name: String,
    pub rename: String,
}