use std::collections::HashMap;
use std::io::{fs, File};
use std::io::fs::PathExtensions;

use mustache::{Data, Template};
use mustache::Data::StrVal;
use rustdoc::html::markdown::Markdown;

use error::{FerrumError, FerrumResult};
use error::ErrorKind::{InvalidPageError, ParserError};
use parser;
use util;

#[deriving(PartialEq, Show)]
pub struct Page {
    header: HashMap<String, String>,
    content: String
}

impl Page {
    pub fn new(header: HashMap<String, String>, content: &str) -> Page {
        Page { header: header, content: content.to_string() }
    }

    pub fn template(&self) -> FerrumResult<&str> {
        match self.header.get(&"template".to_string()) {
            Some(v) => Ok(v.as_slice()),
            None => Err(FerrumError {
                kind: InvalidPageError,
                desc: "Missing template for page",
                detail: None
            })
        }
    }

    pub fn render_to_file<'a, W: Writer>(&self, template: &Template, wr: &mut W, extra_data: &HashMap<String, Data<'a>>) {
        let mut data = HashMap::<String, Data<'a>>::new();

        let content = format!("{}", Markdown(self.content.as_slice()));
        data.insert("content".to_string(), StrVal(content));

        // TODO: find a better way to handle this.
        for (key, value) in self.header.iter() {
            data.insert(key.to_string(), StrVal(value.clone()));
        }
        for (key, value) in extra_data.iter() {
            data.insert(key.to_string(), util::copy_data(value));
        }

        template.render_data(wr, &Data::Map(data));
    }
}

pub fn load_pages_from_disk(pages_path: &Path, criteria: |&Path| -> bool) -> FerrumResult<HashMap<Path, Page>> {
    let mut pages = HashMap::new();

    let mut page_dirs = try!(fs::walk_dir(pages_path));
    for path in page_dirs {
        if !path.is_file() { continue; }
        if !criteria(&path) { continue; }

        // Read the page markdown from the disk.
        let content = try!(File::open(&path).read_to_end());
        let content = String::from_utf8_lossy(content.as_slice());
        let page = match parser::page(content.as_slice()) {
            Ok(page) => page,
            Err(err) => {
                let err = FerrumError {
                    kind: ParserError(err),
                    desc: "Failed to parse a string.",
                    detail: None
                };
                warn!("Failed to read page {}: {}", path.display(), err);
                continue;
            }
        };
        let mut relative_path = path.path_relative_from(pages_path).unwrap();
        relative_path.set_extension("");

        pages.insert(relative_path, page);
    }

    Ok(pages)
}
