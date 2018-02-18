// Copyright (c) 2016, 2018 Nikita Pekin and the ferru contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Ferru is a lightweight, blazing fast static site generator.
//!
//! This binary serves as a frontend to the Ferru static site generation
//! functionality provided by the ferru library.

#![deny(missing_docs)]
#![deny(non_camel_case_types)]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate ferru;
#[macro_use]
extern crate log;
extern crate mustache;

use clap::{App, ArgMatches};
use ferru::Config;

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
        let config = config_from_matches(sub_m);
        match name {
            "build" => ferru::build(&config).unwrap(),
            _ => unreachable!(),
        }
    }
}

fn config_from_matches<'a>(matches: &'a ArgMatches) -> Config<'a> {
    Config {
        source_directory: matches.value_of("source"),
        dest_directory: matches.value_of("destination"),
    }
}
