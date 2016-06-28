use std::borrow::Cow;
use super::*;

#[derive(Clone, Debug)]
pub struct Query<'a> {
    callback: Cow<'a, str>,
    format: Format,
    no_html: bool,
    no_redirect: bool,
    pretty: bool,
    query: Cow<'a, str>,
    skip_disambig: bool,
}


impl<'a> Query<'a> {
    pub fn new<I: Into<Cow<'a, str>>>(query: I) -> Self {
        Query { query: query.into(), ..Self::default() }
    }

    pub fn callback<I: Into<Cow<'a, str>>>(mut self, callback: I) -> Self {
        self.callback = callback.into();
        self
    }

    pub fn format<I: Into<Format>>(mut self, format: I) -> Self {
        self.format = format.into();
        self
    }

    pub fn no_html(mut self) -> Self {
        self.no_html = true;
        self
    }

    pub fn no_redirect(mut self) -> Self {
        self.no_redirect = true;
        self
    }

    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }

    pub fn skip_disambig(mut self) -> Self {
        self.skip_disambig = true;
        self
    }

    pub fn execute(&self) -> DdgResponse {
        unimplemented!()
    }
}

impl<'a> Default for Query<'a> {
    fn default() -> Self {
        Query {
            callback: Cow::Borrowed(""),
            format: Format::default(),
            no_html: false,
            no_redirect: false,
            pretty: false,
            query: Cow::Borrowed(""),
            skip_disambig: false,
        }
    }
}
