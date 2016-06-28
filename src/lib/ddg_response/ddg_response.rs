use super::*;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DdgResponse {
    /// Mapping from `Abstract`
    #[serde(rename="Abstract")]
    pub abstract_head: String,
    #[serde(rename="AbstractText")]
    pub abstract_text: String,
    #[serde(rename="AbstractSource")]
    pub abstract_source: String,
    #[serde(rename="AbstractURL")]
    pub abstract_url: String,
    #[serde(rename="Image")]
    pub image: String,
    #[serde(rename="Heading")]
    pub heading: String,
    #[serde(rename="Answer")]
    pub answer: String,
    #[serde(rename="AnswerType")]
    pub answer_type: String,
    #[serde(rename="Definition")]
    pub definition: String,
    #[serde(rename="DefinitionSource")]
    pub definition_source: String,
    #[serde(rename="DefinitionURL")]
    pub definition_url: String,
    #[serde(rename="RelatedTopics", deserialize_with="topic_or_related_topic")]
    pub related_topics: Vec<RelatedTopic>,
    #[serde(rename="Results")]
    pub results: Vec<DdgResult>,
    /// Mapping from `Type`
    #[serde(rename="Type")]
    pub response_type: Type,
    #[serde(rename="Redirect")]
    pub redirect: String,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn is_valid_response() {
        let json = include_str!("../../../tests/test.json");

        let actual_json: DdgResponse = serde_json::from_str(json).unwrap();

        assert_eq!(actual_json.heading, "Apple");
    }
}
