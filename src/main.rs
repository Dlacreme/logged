//! # LOGGED!
//! `logged` is a modern database system specifically developed to store events.
//! We aim at highly reducing the disk space used by logs. By including the main properties
//! of logs directly in the database system (like the type or the time), not only we can reduce the disk size
//! but we can also optimized the query system.
//!
//! Start: $> logged my_app
//! Run a single query: $> logged my_app -i SELECT date, metadata FROM error
//!

#[macro_use] extern crate pest_derive;
use clap::{Command, Arg, ArgMatches};

const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PACKAGE_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[macro_use] mod log;
mod repl;
mod parser;
mod query;
mod error;

/// We are looking for a few parameters before starting the command-line:
/// - `version`: display the current version and exit
/// - `debug`: also display debug logs
/// - `inline <cmd>`: execute and display the result of $cmd and exit
/// Unless another parameter exits the program, the command-line REPL starts
///
/// If we are executing a query, a `target` application must be provided as first parameters:
/// $> logged my_app -i SELECT date FROM error
/// $> logged my_app
/// @my_app |> ...
///
fn main() {
    let cli_matches = get_cli_matches();

    if cli_matches.occurrences_of("version") > 0 {
        std::process::exit(1);
    }

    if cli_matches.occurrences_of("debug") > 0 {
        log::enable_info_log(true);
    }

    let target = cli_matches.value_of("target").unwrap();

    if cli_matches.occurrences_of("inline") > 0 {
        let cmd = cli_matches.value_of("inline").unwrap();
        return repl::execute_once(target, cmd);
    }

   repl::start(target);
}

fn get_cli_matches() -> ArgMatches {
    Command::new(PACKAGE_NAME)
        .version(PACKAGE_VERSION)
        .author(PACKAGE_AUTHORS)
        .about("Modern database system for logs")
        .arg(Arg::new("target").index(1).required_unless_present("version"))
        .arg(Arg::new("debug").short('d').long("debug").help("Display all the logs - DEBUG PURPOSE"))
        .arg(Arg::new("inline").short('i').long("inline").help("Run a single query").takes_value(true))
        .arg(Arg::new("version").short('v').long("version").help("Display version"))
        .get_matches()
}