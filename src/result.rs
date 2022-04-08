//! Handle a query runtime

use crate::error::Error;

/// Store details about a query and its result
#[derive(Debug)]
pub struct Query {
    command: String,
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
    pub fn new(target: String, cmd: String) -> Self {
        Self {
            command: cmd,
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
