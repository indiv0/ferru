// TODO: write a description.
//! Ferrum is a ...

#![deny(non_camel_case_types)]
#![feature(collections, core, old_io, old_path, plugin)]
#![plugin(peg_syntax_ext)]

extern crate getopts;
#[macro_use] extern crate log;
extern crate mustache;

use std::os;

use getopts::Options;

mod error;
mod ferrum;
mod parser;
mod document;
mod template;
mod util;

fn main() {
    // Setup the possible opts.
    let mut opts = Options::new();
    opts.optopt("s", "source", "set source directory", "NAME");
    opts.optopt("d", "destination", "set destination directory", "NAME");
    opts.optflag("h", "help", "print this help menu");

    // Get the arguments and program name.
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    // Match the opts.
    let matches = match opts.parse(args.tail()) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Check if the help opt is present.
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Retrieve the command.
    let command = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    match &*command {
        "build" => ferrum::build(matches),
        _ => {
            print_usage(&program, opts);
            return;
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
