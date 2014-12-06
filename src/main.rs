// TODO: write a description.
//! Ferrum is a ...

#![deny(non_camel_case_types)]
#![feature(phase)]

extern crate getopts;
extern crate http;
#[phase(plugin, link)] extern crate log;
extern crate nickel;
#[phase(plugin)]extern crate peg_syntax_ext;
extern crate rustdoc;
extern crate serialize;
extern crate toml;

use getopts::{optopt, optflag, getopts, OptGroup};
use nickel::{Nickel, HttpRouter, StaticFilesHandler};
use std::os;

use markdown_page_handler::MarkdownPageHandler;

mod config;
mod controllers;
mod error;
mod markdown_page_handler;
mod parser;
mod post;
mod util;

static DEFAULT_CONFIG_PATH: &'static str = "./config.toml";

fn main() {
    // Get the arguments.
    let args: Vec<String> = os::args();
    // Get the program name.
    let program = args[0].clone();

    // Setup the possible opts.
    let opts = &[
        optopt("c", "", "config file name", "NAME"),
        optflag("h", "help", "print this help menu")
    ];
    // Match the opts.
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    // Check if the help opt is present.
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    // Get the config opt.
    let config_path = match matches.opt_str("c") {
        Some(v) => v,
        None => DEFAULT_CONFIG_PATH.to_string()
    };

    // Load configuration.
    let config = match config::Config::new(config_path.as_slice()) {
        Ok(v) => v,
        Err(_) => {
            // Create a default config file if it doesn't exist.
            warn!("Failed to read config.toml; creating from defaults.");
            match config::write_default_config(DEFAULT_CONFIG_PATH) {
                Ok(_) => {},
                Err(e) => panic!(e)
            };
            config::default_config()
        }
    };

    // Initialize the server and routing.
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    // Enable routes for index and blog pages.
    router.get("/", controllers::root_handler);
    //router.get("/blog/:post_year/:post_id", controllers::get_blog_post);
    //router.get("/:page_id", controllers::get_page);

    // Attach the router to the server.
    server.utilize(router);
    // Enable Markdown page serving.
    server.utilize(MarkdownPageHandler::new(config.content_path().as_slice(), "assets/templates/post.tpl"));
    // Enable static file serving.
    // NOTE: outside the dev environment, use nginx instead.
    // TODO: ensure this only works in testing (compiler flags?).
    server.utilize(StaticFilesHandler::new(config.static_path().as_slice()));

    // Register error handling routes.
    server.handle_error(controllers::custom_errors);

    // Begin listening on provided IP and port.
    let ip_addr = match config.ip_addr() {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    server.listen(ip_addr, config.port());
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-c\t\tConfig file");
    println!("-h --help\tUsage");
}
