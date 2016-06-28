use self::Type::*;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Type {
    #[serde(rename="A")]
    Article,
    #[serde(rename="D")]
    Disambiguation,
    #[serde(rename="C")]
    Category,
    #[serde(rename="N")]
    Name,
    #[serde(rename="E")]
    Exclusive,
    #[serde(rename="")]
    Nothing,
}

impl Default for Type {
    fn default() -> Self {
        Nothing
    }
}
