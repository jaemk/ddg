use serde::{de, Deserializer};

/// An icon associated with a URL. Icon's can be bigger than the dimensions
/// specified in the struct.
#[derive(Clone, Debug, Deserialize)]
pub struct Icon {
    /// The **suggested** height of the icon.
    #[serde(rename="Height", deserialize_with="deserialize_u64_or_empty_string")]
    height: u64,
    /// The url to the icon.
    #[serde(rename="URL")]
    url: String,
    /// The **suggested** width of the icon.
    #[serde(rename="Width", deserialize_with="deserialize_u64_or_empty_string")]
    width: u64,
}

// from: http://stackoverflow.com/questions/37870428/serde-handle-value-being-two-different-types
struct DeserializeU64OrEmptyStringVisitor;

impl de::Visitor for DeserializeU64OrEmptyStringVisitor {
    type Value = u64;

    fn visit_u64<E>(&mut self, v: u64) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v)
    }

    fn visit_str<E>(&mut self, v: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        if v == "" {
            Ok(0)
        } else {
            Err(E::invalid_value("got a non-empty string"))
        }
    }
}

fn deserialize_u64_or_empty_string<D>(deserializer: &mut D) -> Result<u64, D::Error>
    where D: Deserializer
{
    deserializer.deserialize(DeserializeU64OrEmptyStringVisitor)
}
