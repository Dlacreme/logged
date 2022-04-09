//! Handle a query runtime

use crate::error::Error;
use crate::parser::{parse, ParsedQuery};

/// Store details about a query and its result
#[derive(Debug)]
pub struct Query {
    target: String,
    raw: String,
    parsed: ParsedQuery,
    error: Option<Error>,
    rows: Vec<Row>,
}

#[derive(Debug)]
struct Row {
    label: String,
    value: String,
}

impl Query {
    /// Create a new query
    pub fn new(target: String, query: String) -> Self {
        let parsed = parse(query.as_str());
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
