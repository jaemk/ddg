use super::Icon;

#[derive(Clone, Debug, Deserialize)]
pub struct DdgResult {
    #[serde(rename="Result")]
    result: String,
    #[serde(rename="FirstURL")]
    first_url: String,
    #[serde(rename="Icon")]
    icon: Icon,
    #[serde(rename="Text")]
    text: String,
}
