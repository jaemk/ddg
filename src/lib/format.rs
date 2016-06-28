use std::borrow::Cow;
use self::Format::*;

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Json,
    Xml,
    Html,
}

impl Default for Format {
    fn default() -> Self {
        Html
    }
}

impl From<String> for Format {
    fn from(original: String) -> Self {
        Self::from(&*original)
    }
}

impl<'a> From<&'a str> for Format {
    fn from(original: &str) -> Self {
        match original {
            "json" => Json,
            "JSON" => Json,
            "Json" => Json,
            "xml" => Xml,
            "XML" => Xml,
            "Xml" => Xml,
            _ => Html,
        }
    }
}

impl<'a> From<Cow<'a, Format>> for Format {
    fn from(original: Cow<'a, Format>) -> Self {
        match original {
            Cow::Borrowed(&format) => format,
            Cow::Owned(format) => format,
        }
    }
}
