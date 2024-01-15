use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProblemDetails {
    pub title: String,
    pub detail: Option<String>,
    pub id: Uuid,
    pub status: u16,
    #[serde(alias = "type")]
    pub _type: Option<String>,
    pub instance: Option<String>
}

impl ProblemDetails {
    pub fn new(status: u16, title: String) -> Self {
        Self {
            status,
            title,
            id: Uuid::new_v4(),
            detail: None,
            _type: None,
            instance: None,
        }
    }

    pub fn with_detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn with_type(mut self, value: String) -> Self {
        self._type = Some(value);
        self
    }

    pub fn with_instance(mut self, instance: String) -> Self {
        self.instance = Some(instance);
        self
    }
}