use std::fmt;

use super::Icon;
use serde::de;

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

impl de::Deserialize for RelatedTopic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer
    {
        deserializer.deserialize(Visitor)
    }
}

struct Visitor;

impl de::Visitor for Visitor {
    type Value = RelatedTopic;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a object containing with Topics, FirstURL, Result properties")
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<RelatedTopic, V::Error>
        where V: de::MapVisitor
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
        val
    }
}

/// An link associated with abstract.
#[derive(Clone, Debug, Deserialize)]
pub struct TopicResult {
    /// First URL for the Result.
    #[serde(rename="FirstURL")]
    pub first_url: String,
    /// Icon associated with `first_url`.
    #[serde(rename="Icon")]
    pub icon: Icon,
    /// HTML link(s) to external site(s).
    #[serde(rename="Result")]
    pub result: String,
    /// Text from `first_url`.
    #[serde(rename="Text")]
    pub text: String,
}


/// Name, and Vec of `TopicResult`s.
#[derive(Clone, Debug, Deserialize)]
pub struct Topic {
    /// vec of external links associated with abstract.
    #[serde(rename="Topics")]
    pub topics: Vec<RelatedTopic>,
    /// The name of the topic. for example if the query was Apple, it could be
    /// a topic about botany, or companies.
    #[serde(rename="Name")]
    pub name: String,
}
