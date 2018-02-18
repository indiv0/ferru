// Copyright (c) 2016, 2018 Nikita Pekin and the ferru contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]
#![deny(non_camel_case_types)]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

//! Ferru is a lightweight, blazing fast static site generator.
//!
//! This crate provides a static site generator which can read in a series of
//! templates and documents. It can then parse the documents to separate them
//! into a body and a header. Using
//! [rust-mustache](https://github.com/nickel-org/rust-mustache) it then renders
//! the attributes in the document header as the data to be templated in.
//! It can also then render a final output by rendering the compiled string as
//! the data for another template file.
//!
//! Ferru can also copy static files over to the directory of the generated
//! website.

#[macro_use]
extern crate log;
extern crate mustache;

pub use config::Config;
pub use document::{load_documents_from_disk, Header, Document};
pub use error::{Error, Result};
pub use ferru::build;
pub use template::{load_templates_from_disk, TemplateMap};

pub mod config;
mod error;
mod ferru;
mod parser;
pub mod document;
mod template;
mod util;
