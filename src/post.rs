use std::collections::HashMap;

#[deriving(PartialEq, Show)]
pub struct Post {
    header: HashMap<String, String>,
    content: String
}

impl Post {
    pub fn new(header: HashMap<String, String>, content: &str) -> Post {
        Post { header: header, content: content.to_string() }
    }
}
