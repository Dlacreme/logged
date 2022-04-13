//! Handle a query runtime

use crate::error::Error;
use crate::parser::{parse, ParsedQuery};

/// Store details about a query and its result
#[derive(Debug)]
pub struct Query<'a> {
    target: &'static str,
    raw: &'a str,
    parsed: ParsedQuery,
    error: Option<Error>,
    rows: Vec<Row<'a>>,
}

#[derive(Debug)]
struct Row<'a> {
    label: &'a str,
    value: &'a str,
}

impl<'a> Query<'a> {
    /// Create a new query
    pub fn new(target: &'static str, query: &'a str) -> Self {
        let parsed = parse(query);
        Self {
            target: target,
            raw: query,
            parsed: ParsedQuery::Undefined,
            error: None,
            rows: Vec::new(),
        }
    }

    /// Format a query as a string (either display the error or formatted result)
    pub fn as_string(&self) -> String {
        if self.error.is_some() {
            return format!("ERROR: {:?}", self.error);
        }
        return format!("{:?}", self.rows)
    }
}
