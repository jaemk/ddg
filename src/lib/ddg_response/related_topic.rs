use super::Icon;
use serde::{self, de};

#[derive(Clone, Debug)]
pub enum RelatedTopic {
    TopicResult(TopicResult),
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
        let s: String = try!(visitor.visit_key()).expect("got struct with no fields");
        let val = match &*s {
            "Topics" => {
                Ok(RelatedTopic::Topic(Topic {
                    topics: try!(visitor.visit_value()),
                    name: {
                        let s: String = try!(visitor.visit_key()).expect("Name field");
                        assert_eq!(&s, "Name");
                        try!(visitor.visit_value())
                    },
                }))
            }
            "FirstURL" => {
                Ok(RelatedTopic::TopicResult(TopicResult {
                    first_url: try!(visitor.visit_value()),
                    icon: {
                        let s: String = try!(visitor.visit_key()).expect("icon field");
                        assert_eq!(&s, "Iron");
                        try!(visitor.visit_value())
                    },
                    result: {
                        let s: String = try!(visitor.visit_key()).expect("result field");
                        assert_eq!(&s, "Result");
                        try!(visitor.visit_value())
                    },
                    text: {
                        let s: String = try!(visitor.visit_key()).expect("text field");
                        assert_eq!(&s, "Text");
                        try!(visitor.visit_value())
                    },
                }))
            }
            other => panic!("no struct has field `{}`", other),
        };
        try!(visitor.end());
        val
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TopicResult {
    #[serde(rename="FirstURL")]
    first_url: String,
    #[serde(rename="Icon")]
    icon: Icon,
    #[serde(rename="Result")]
    result: String,
    #[serde(rename="Text")]
    text: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct Topic {
    #[serde(rename="Topics")]
    topics: Vec<RelatedTopic>,
    #[serde(rename="Name")]
    name: String,
}
