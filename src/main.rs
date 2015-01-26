// TODO: write a description.
//! Ferrum is a ...

#![deny(non_camel_case_types)]
#![feature(plugin)]

extern crate getopts;
#[macro_use] extern crate log;
extern crate mustache;
#[plugin] extern crate peg_syntax_ext;

use std::os;

use getopts::{getopts, optopt, optflag, short_usage, usage};

mod error;
mod ferrum;
mod parser;
mod document;
mod template;
mod util;

fn main() {
    // Setup the possible opts.
    let opts = &[
        optopt("s", "source", "set source directory", "NAME"),
        optopt("d", "destination", "set destination directory", "NAME"),
        optflag("h", "help", "print this help menu")
    ];

    let instructions = "Usage: ferrum [command]";

    // Get the arguments and program name.
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    // Match the opts.
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f) }
    };

    // Check if the help opt is present.
    if matches.opt_present("h") {
        println!("{}", usage(&*program, opts));
        return;
    }

    // Retrieve the command.
    let command = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("{}", short_usage(instructions, opts));
        return;
    };

    match &*command {
        "build" => ferrum::build(matches),
        _ => {
            println!("{}", short_usage(instructions, opts));
            return;
        }
    }
}
