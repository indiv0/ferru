// TODO: write a description.
//! Ferrum is a ...

#![deny(non_camel_case_types)]
#![feature(phase)]

extern crate getopts;
#[phase(plugin, link)] extern crate log;
extern crate mustache;
#[phase(plugin)]extern crate peg_syntax_ext;
extern crate rustdoc;

use getopts::{getopts, optopt, optflag, short_usage, usage, Matches};
use std::os;
use std::io;
use std::io::fs;
use std::io::fs::PathExtensions;

mod error;
mod parser;
mod document;
mod template;
mod util;

static DEFAULT_SOURCE_PATH: &'static str = "./";
static DEFAULT_DEST_PATH: &'static str = "./_site/";

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
        Err(f) => { panic!(f.to_string()) }
    };

    // Check if the help opt is present.
    if matches.opt_present("h") {
        println!("{}", usage(program.as_slice(), opts));
        return;
    }

    // Retrieve the command.
    let command = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("{}", short_usage(instructions, opts));
        return;
    };

    match command.as_slice() {
        "build" => build(matches),
        _ => {
            println!("{}", short_usage(instructions, opts));
            return;
        }
    }
}

fn build(matches: Matches) {
    // Get the source path opt.
    let source = match matches.opt_str("s") {
        Some(v) => Path::new(v),
        None => Path::new(DEFAULT_SOURCE_PATH)
    };
    if !source.exists() {
        panic!("Source directory \"{}\" does not exist.", source.display());
    }

    // Get the destination path opt.
    let dest = match matches.opt_str("d") {
        Some(v) => Path::new(v),
        None => Path::new(DEFAULT_DEST_PATH)
    };

    if !dest.exists() {
        println!("Destination directory \"{}\" does not exist, creating.", dest.display());
    } else {
        println!("Cleaning destination directory \"{}\".", dest.display());
        fs::rmdir_recursive(&dest).is_ok();
    }
    fs::mkdir(&dest, io::USER_RWX).is_ok();

    // Load the templates.
    let templates = match template::load_templates_from_disk(&source, |p| -> bool {
        !p.filename_str().unwrap().starts_with(".") &&
        p.extension_str().unwrap() == "tpl"
    }) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to read templates: {}", e);
            return;
        }
    };

    // Copy all non-template and non-document content.
    if source != dest {
        match util::copy_recursively(&source, &dest, |p| -> bool {
            !p.filename_str().unwrap().starts_with(".") &&
            p != &dest &&
            p.path_relative_from(&source).unwrap().as_str().unwrap() != "_posts" &&
            p.path_relative_from(&source).unwrap().as_str().unwrap() != "_templates"
        }) {
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    debug!("Loading documents from disk");
    let documents = document::load_documents_from_disk(&source.join("_posts"), |p| -> bool {
        !p.filename_str().unwrap().starts_with(".")
    }).unwrap();

    debug!("Rendering documents");
    for (key, document) in documents.into_iter() {
        let new_dest = dest.join(&key);
        document.render_to_file(&new_dest, &templates).is_ok();
    }
}
