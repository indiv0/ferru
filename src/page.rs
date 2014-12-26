use std::collections::HashMap;
use std::io::{fs, File};
use std::io::fs::PathExtensions;

use mustache::Template;
use rustdoc::html::markdown::Markdown;

use error::{FerrumError, FerrumResult};
use error::ErrorKind::{InvalidPageError, MustacheError, ParserError};
use parser;

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

    pub fn render_to_file<W: Writer>(&self, template: &Template, wr: &mut W) -> FerrumResult<()> {
        let content = format!("{}", Markdown(self.content.as_slice()));
        let mut data = HashMap::<&str, &str>::new();
        data.insert("content", content.as_slice());

        // TODO: find a better way to handle this.
        for (key, value) in self.header.iter() {
            data.insert(key.as_slice(), value.as_slice());
        }

        match template.render(wr, &data) {
            Ok(_) => Ok(()),
            Err(e) => Err(FerrumError {
                kind: MustacheError,
                desc: "Mustache templating error",
                detail: Some(format!("{}", e))
            })
        }
    }
}

pub fn load_pages_from_disk(source: &Path, criteria: |&Path| -> bool) -> FerrumResult<HashMap<Path, Page>> {
    let mut pages = HashMap::new();

    let pages_path = source.join("_pages");
    let mut page_dirs = try!(fs::walk_dir(&pages_path));
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
        let mut relative_path = path.path_relative_from(&pages_path).unwrap();
        relative_path.set_extension("");

        pages.insert(relative_path, page);
    }

    Ok(pages)
}
