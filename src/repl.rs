//! Handle the main loop of events. Also provide a helper to run a single query

use std::io::{stdout, stdin, Write};
use crate::query::Query;
use crate::log;

/// Starts the REPL. It waits for a query on stdin and then display the results
pub fn start(target: &str) {
    let mut line: String = String::new();
    let mut query: Query = Query::new(target, line);
    while line != "@exit" {
        query = reset_query();
        print_prompt(target);
        line = read_line(line);
        // log::debug(format!("Execute query '{}'", line));
        query = Query::new(target, line.as_str());
        query = execute(query);
        print_result(query);
    }
}

/// Execute a single query and display the result on stdin
pub fn execute_once(target: &str, line: &str) {
    log::debug(format!("Execute query '{}'", line));
    let mut query = Query::new(target, line);
    query = execute(query);
    print_result(query);
}

fn reset_query(query: Query, target: &str, line: &str) -> Query {

}

fn read_line(mut buffer: String) -> String {
    stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn print_prompt(target: &str) {
    print!("@{}|> ", target);
    stdout().flush().unwrap();
}

fn execute(query: Query) -> Query {
    return query;
}

fn print_result(res: Query) {
    print!("{}", res.as_string());
    stdout().flush().unwrap();
}