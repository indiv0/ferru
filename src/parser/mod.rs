// Copyright (c) 2016, 2018 Nikita Pekin and the ferru contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde_yaml;

pub use self::error::{Error, Result};

use std::collections::BTreeMap;

use document::{Document, Header};

mod error;

const HEADER_SEPARATOR: &'static str = "---\n";

pub fn parse_document(content: &str) -> Result<Document> {
    // Create a mutable local copy of the content, which we will modify to
    // contain just the document body if a header is present.
    let mut content = content;

    // If a header is defined in the file, fill the header hashmap with the
    // header attributes.
    // Otherwise, initialize an empty `HashMap`.
    let header = if content.contains(HEADER_SEPARATOR) {
        // Split the suplied content string in two, at the location of the
        // `HEADER_SEPARATOR`.
        let mut content_split = content.splitn(2, HEADER_SEPARATOR);

        // Retrieve the strings containing the header and the content from the
        // `SplitN` iterator.
        let header_string = content_split.next().unwrap_or("");
        content = content_split.next().unwrap_or("");

        parse_header(header_string)?
    } else {
        BTreeMap::new()
    };

    Ok(Document::new(header, content))
}

fn parse_header(s: &str) -> Result<Header> {
    // Parse the YAML from the header string.
    Ok(serde_yaml::from_str(s)?)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use document::Document;
    use parser::{parse_document, parse_header, Error};
    use parser::yaml_rust::Yaml;

    #[test]
    fn test_parse_invalid_header_key() {
        assert_eq!(
            parse_header("a"),
            Err(Error::InvalidHeaderFormat("a".to_owned()))
        );
        assert_eq!(
            parse_header("a "),
            Err(Error::InvalidHeaderFormat("a ".to_owned()))
        );
        assert_eq!(
            parse_header(": "),
            Err(Error::InvalidHeaderKey(Yaml::Null))
        );
        assert_eq!(
            parse_header(": \n"),
            Err(Error::InvalidHeaderKey(Yaml::Null))
        );
        assert_eq!(
            parse_header("title:sometitle"),
            Err(Error::InvalidHeaderFormat("title:sometitle".to_owned()))
        );
        assert_eq!(
            parse_header("1234: sometitle"),
            Err(Error::InvalidHeaderKey(Yaml::Integer(1234)))
        );
    }

    #[test]
    fn test_parse_invalid_header_value() {
        assert_eq!(
            parse_header("key: "),
            Err(Error::InvalidHeaderValue(Yaml::Null))
        );
        assert_eq!(
            parse_header("key: \n"),
            Err(Error::InvalidHeaderValue(Yaml::Null))
        );
        assert_eq!(
            parse_header("key: 1234"),
            Err(Error::InvalidHeaderValue(Yaml::Integer(1234)))
        );
    }

    #[test]
    fn test_parse_header() {
        let map = {
            let mut map = HashMap::new();
            map.insert("title".to_owned(), "sometitle".to_owned());
            map.insert("date".to_owned(), "2014-01-01".to_owned());
            Ok(map)
        };

        assert_eq!(&parse_header("title: sometitle\ndate: 2014-01-01"), &map);
        assert_eq!(&parse_header("title: sometitle\ndate: 2014-01-01\n"), &map);
    }

    #[test]
    fn test_parse_empty_document() {
        assert_eq!(parse_document(""), Ok(Document::new(HashMap::new(), "")));
    }

    #[test]
    fn test_parse_document_empty_header_and_body() {
        assert_eq!(parse_document("---\n"), Ok(Document::new(HashMap::new(), "")));
        assert_eq!(parse_document("\n---\n"), Ok(Document::new(HashMap::new(), "")));
        assert_eq!(parse_document(" ---\n"), Ok(Document::new(HashMap::new(), "")));
    }

    #[test]
    fn test_parse_document_empty_header() {
        assert_eq!(parse_document("---\nsome random content"), Ok(Document::new(HashMap::new(), "some random content")));
        assert_eq!(parse_document("\n---\nsome random content"), Ok(Document::new(HashMap::new(), "some random content")));
        assert_eq!(parse_document(" ---\nsome random content"), Ok(Document::new(HashMap::new(), "some random content")));
    }

    #[test]
    fn test_parse_document_empty_body() {
        let header = {
            let mut header = HashMap::new();
            header.insert("title".to_owned(), "my title".to_owned());
            header
        };

        assert_eq!(parse_document("title: my title\n---\n"), Ok(Document::new(header, "")));
    }

    #[test]
    fn test_parse_document() {
        assert!(parse_document("title: sometitle\ndate: 2014-01-01\n---\n\nthis is a post.").is_ok());

        let header = {
            let mut header = HashMap::new();
            header.insert("title".to_owned(), "sometitle".to_owned());
            header.insert("date".to_owned(), "2014-01-01".to_owned());
            header
        };

        let post = Document::new(header, "this is a post.\nwith multiple lines!");

        assert_eq!(parse_document("title: sometitle\ndate: 2014-01-01\n---\nthis is a post.\nwith multiple lines!"), Ok(post));
    }
}
