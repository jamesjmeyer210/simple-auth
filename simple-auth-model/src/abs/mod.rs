pub(crate) trait IsValid<T> {
    fn is_valid(value: T) -> bool;
}

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl AsBytes for String {
    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

pub trait AsJson {
    fn as_json(&self) -> Result<String,serde_json::Error>;
}

impl <T>AsJson for T where T: serde::Serialize {
    fn as_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}