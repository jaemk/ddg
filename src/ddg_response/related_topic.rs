use super::Icon;
use serde;

/// Internal link(s) to related topics associated with abstract. A result could
/// either be a single `TopicResult`, or a `Topic` containing multiple
/// `TopicResult`s in a certain area of interest.
#[derive(Debug, Clone)]
pub enum RelatedTopic {
    /// The link had a single topic.
    TopicResult(TopicResult),
    /// The link had a whole topic category.
    Topic(Topic),
}

impl serde::de::Deserialize for RelatedTopic {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        deserializer.deserialize(Visitor)
    }
}

struct Visitor;

impl serde::de::Visitor for Visitor {
    type Value = RelatedTopic;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<RelatedTopic, V::Error>
        where V: serde::de::MapVisitor
    {
        let s: String = visitor.visit_key()?.expect("got struct with no fields");
        let val = match &*s {
            "Topics" => {
                Ok(RelatedTopic::Topic(Topic {
                    topics: visitor.visit_value()?,
                    name: {
                        let s: String = visitor.visit_key()?.expect("Name field");
                        assert_eq!(&s, "Name");
                        visitor.visit_value()?
                    },
                }))
            }
            "FirstURL" => {
                Ok(RelatedTopic::TopicResult(TopicResult {
                    first_url: visitor.visit_value()?,
                    icon: {
                        let s: String = visitor.visit_key()?.expect("icon field");
                        assert_eq!(&s, "Icon");
                        visitor.visit_value()?
                    },
                    result: {
                        let s: String = visitor.visit_key()?.expect("result field");
                        assert_eq!(&s, "Result");
                        visitor.visit_value()?
                    },
                    text: {
                        let s: String = visitor.visit_key()?.expect("text field");
                        assert_eq!(&s, "Text");
                        visitor.visit_value()?
                    },
                }))
            },
            "Result" => {
                Ok(RelatedTopic::TopicResult(TopicResult {
                    result: visitor.visit_value()?,
                    icon: {
                        let s: String = visitor.visit_key()?.expect("icon field");
                        assert_eq!(&s, "Icon");
                        visitor.visit_value()?
                    },
                    first_url: {
                        let s: String = visitor.visit_key()?.expect("result field");
                        assert_eq!(&s, "FirstURL");
                        visitor.visit_value()?
                    },
                    text: {
                        let s: String = visitor.visit_key()?.expect("text field");
                        assert_eq!(&s, "Text");
                        visitor.visit_value()?
                    },
                }))
            },
            other => panic!("no struct has field `{}`", other),
        };
        visitor.end()?;
        val
    }
}

/// An link associated with abstract.
#[derive(Clone, Debug, Deserialize)]
pub struct TopicResult {
    /// First URL for the Result.
    #[serde(rename="FirstURL")]
    first_url: String,
    /// Icon associated with `first_url`.
    #[serde(rename="Icon")]
    icon: Icon,
    /// HTML link(s) to external site(s).
    #[serde(rename="Result")]
    result: String,
    /// Text from `first_url`.
    #[serde(rename="Text")]
    text: String,
}


/// Name, and Vec of `TopicResult`s.
#[derive(Clone, Debug, Deserialize)]
pub struct Topic {
    /// vec of external links associated with abstract.
    #[serde(rename="Topics")]
    topics: Vec<RelatedTopic>,
    /// The name of the topic. for example if the query was Apple, it could be
    /// a topic about botany, or companies.
    #[serde(rename="Name")]
    name: String,
}
