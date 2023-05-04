use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub id: u64,
    pub message: Protocol,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Protocol {
    #[serde(with = "self")]
    Error(openxr_sys::Result),
}

fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<openxr_sys::Result, D::Error> {
    todo!()
}

fn serialize<T, S: Serializer>(input: &T, serializer: S) -> Result<S::Ok,  S::Error> {
    todo!()
}