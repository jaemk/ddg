use std::fmt;

use super::Icon;
use serde::de;

/// Internal link(s) to related topics associated with abstract. A result could
/// either be a single `TopicResult`, or a `Topic` containing multiple
/// `TopicResult`s in a certain area of interest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelatedTopic {
    /// The link had a single topic.
    TopicResult(TopicResult),
    /// The link had a whole topic category.
    Topic(Topic),
}

impl<'de> de::Deserialize<'de> for RelatedTopic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        deserializer.deserialize_map(Visitor)
    }
}

struct Visitor;

macro_rules! serialise {
    ($builder:ident, $visitor:ident) => {
        match &*$visitor.next_key::<String>().expect("valid key").expect("field") {
            "Icon" => {
                $builder.icon = $visitor.next_value()?;
            }
            "Result" => {
                $builder.result = $visitor.next_value()?;
            }
            "FirstURL" => {
                $builder.first_url = $visitor.next_value()?;
            }
            "Text" => {
                $builder.text = $visitor.next_value()?;
            }
            other => panic!("no struct has key `{}`", other),
        }
    }
}

impl<'de> de::Visitor<'de> for Visitor {
    type Value = RelatedTopic;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a object containing with Topics, FirstURL, Result properties")
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<RelatedTopic, V::Error>
        where V: de::MapAccess<'de>
        {
            let s: String = visitor.next_key()?.expect("got struct with no fields");
            let val = match &*s {
                "Topics" => {
                    Ok(RelatedTopic::Topic(Topic {
                        topics: visitor.next_value()?,
                        name: {
                            let s: String = visitor.next_key()?.expect("Name field");
                            assert_eq!(&s, "Name");
                            visitor.next_value()?
                        },
                    }))
                },

                "Name" => {
                    Ok(RelatedTopic::Topic(Topic {
                        name: visitor.next_value()?,
                        topics: {
                            let s: String = visitor.next_key()?.expect("Topics field");
                            assert_eq!(&s, "Topics");
                            visitor.next_value()?
                        },
                    }))
                },

                "FirstURL" => {
                    let mut builder = TopicResultBuilder::default();
                    let first_url = visitor.next_value()?;
                    serialise!(builder, visitor);
                    serialise!(builder, visitor);
                    serialise!(builder, visitor);

                    Ok(RelatedTopic::TopicResult(TopicResult {
                        first_url: first_url,
                        icon: builder.icon.expect("Icon field"),
                        result: builder.result.expect("Result field"),
                        text: builder.text.expect("Text field"),
                    }))
                },

                "Result" => {
                    let mut builder = TopicResultBuilder::default();
                    let result = visitor.next_value()?;
                    serialise!(builder, visitor);
                    serialise!(builder, visitor);
                    serialise!(builder, visitor);

                    Ok(RelatedTopic::TopicResult(TopicResult {
                        first_url: builder.first_url.expect("Result field"),
                        icon: builder.icon.expect("Icon field"),
                        result: result,
                        text: builder.text.expect("Text field"),
                    }))
                },

                "Icon" => {
                    let mut builder = TopicResultBuilder::default();
                    let icon = visitor.next_value()?;
                    serialise!(builder, visitor);
                    serialise!(builder, visitor);
                    serialise!(builder, visitor);

                    Ok(RelatedTopic::TopicResult(TopicResult {
                        first_url: builder.first_url.expect("Result field"),
                        icon: icon,
                        result: builder.result.expect("Icon field"),
                        text: builder.text.expect("Text field"),
                    }))
                },

                "Text" => {
                    let mut builder = TopicResultBuilder::default();
                    let text = visitor.next_value()?;
                    serialise!(builder, visitor);
                    serialise!(builder, visitor);
                    serialise!(builder, visitor);

                    Ok(RelatedTopic::TopicResult(TopicResult {
                        first_url: builder.first_url.expect("Result field"),
                        icon: builder.icon.expect("Icon field"),
                        result: builder.result.expect("Result field"),
                        text: text,
                    }))
                },

                other => panic!("no struct has field `{}`", other),
            };
            val
        }
}

/// An link associated with abstract.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
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

// Internal struct for handling non guarenteed order of TopicResult.
#[derive(Default)]
struct TopicResultBuilder {
    pub first_url: Option<String>,
    pub icon: Option<Icon>,
    pub result: Option<String>,
    pub text: Option<String>,
}


/// Name, and Vec of `TopicResult`s.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Topic {
    /// vec of external links associated with abstract.
    #[serde(rename="Topics")]
    pub topics: Vec<RelatedTopic>,
    /// The name of the topic. for example if the query was Apple, it could be
    /// a topic about botany, or companies.
    #[serde(rename="Name")]
    pub name: String,
}
