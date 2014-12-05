use rustdoc::html::markdown::Markdown;
use std::collections::HashMap;
use std::io::File;

use error::{FerrumError, FerrumResult};
use error::ErrorKind::ParserError;
use parser;

#[deriving(PartialEq, Show)]
pub struct Post {
    header: HashMap<String, String>,
    content: String
}

impl Post {
    pub fn new(header: HashMap<String, String>, content: &str) -> Post {
        Post { header: header, content: content.to_string() }
    }

    pub fn render(&self) -> String {
        format!("{}", Markdown(self.content.as_slice()))
    }
}

pub fn load_from_disk(path_str: &str) -> FerrumResult<Post> {
    let path = Path::new(path_str);

    // Read the post markdown from the disk.
    let content = try!(File::open(&path).read_to_end());
    // TODO: replace this unwrap() with a try!
    let content = String::from_utf8(content).unwrap();
    match parser::post(content.as_slice()) {
        Ok(post) => Ok(post),
        Err(err) => {
            Err(FerrumError {
                kind: ParserError(err),
                desc: "Failed to parse a string.",
                detail: None
            })
        }
    }
}
