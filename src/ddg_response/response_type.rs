use self::Type::*;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Type {
    /// An Article about the answer.
    #[serde(rename="A")]
    Article,
    /// A Disambiguation of the answer.
    #[serde(rename="D")]
    Disambiguation,
    /// Category
    #[serde(rename="C")]
    Category,
    /// Name
    #[serde(rename="N")]
    Name,
    /// Exclusive
    #[serde(rename="E")]
    Exclusive,
    /// Instant Answer has no type.
    #[serde(rename="")]
    Nothing,
}

impl Default for Type {
    fn default() -> Self {
        Nothing
    }
}
