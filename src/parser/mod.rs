//! Use PEST to parse a query and return a ParsedQuery

use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct QueryParser;

#[derive(Debug)]
enum Arg {

}

/// Define all the expected query. If none match, return `Undefined`
#[derive(Debug)]
pub enum ParsedQuery {
    Undefined,
    Select(String, Vec<Arg>)
}

/// parse a query into a ParsedQuery
pub fn parse(query: &str) -> Result<ParsedQuery, String> {
    match QueryParser::parse(Rule::file, query) {
        Ok(mut next) => {
            println!("NEXT > '{:?}'", next);
            Ok(ParsedQuery::Undefined)
        },
        Err(err) => {
            Err(format!("Invalid input. See inner error: {:?}", err))
        }
    }
}
