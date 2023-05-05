use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

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

#[doc(hidden)]
struct XrResultVisitor;

impl<'de> de::Visitor<'de> for XrResultVisitor {
    type Value = openxr_sys::Result;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i32<E: de::Error>(self, value: i32) -> Result<Self::Value, E> {
        Ok(openxr_sys::Result::from_raw(value))
    }
}


fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<openxr_sys::Result, D::Error> {
    deserializer.deserialize_i32(XrResultVisitor)
}

fn serialize<S: Serializer>(input: &openxr_sys::Result, serializer: S) -> Result<S::Ok,  S::Error> {
    serializer.serialize_i32(input.into_raw())
}