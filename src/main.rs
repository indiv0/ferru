// TODO: write a description.
//! Ferrum is a ...

#![deny(non_camel_case_types)]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate mustache;

use clap::{App, ArgMatches};

mod error;
mod ferrum;
mod parser;
mod document;
mod template;
mod util;

fn main() {
    env_logger::init()
        .expect("Failed to initialize env_logger");

    // Create the CLI.
    let yml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    // Run the program.
    run(matches);
}

fn run(m: ArgMatches) {
    // Match the raw subcommand, and get its sub-matches "m".
    if let (name, Some(sub_m)) = m.subcommand() {
        match name {
            "build" => ferrum::build(sub_m).unwrap(),
            _ => unreachable!(),
        }
    }
}
