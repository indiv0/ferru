pub use self::grammar::{header, page};

// grammar.rustpeg contains the parsing expression grammar needed in order to
// parse posts.
peg_file! grammar("grammar.rustpeg");

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use page::Page;

    #[test]
    fn test_invalid_header() {
        assert!(super::header(": ").is_err());
        assert!(super::header(": \n").is_err());
        assert!(super::header("title:sometitle").is_err());
    }

    #[test]
    fn test_header() {
        let mut map = HashMap::new();
        map.insert("title".to_string(), "sometitle".to_string());
        map.insert("date".to_string(), "2014-01-01".to_string());
        let map = map;

        assert_eq!(super::header("title: sometitle\ndate: 2014-01-01\n"), Ok(map));

        assert!(super::header("title: ").is_ok());
        assert!(super::header("title: \n").is_ok());
    }

    #[test]
    fn test_invalid_page() {
        assert!(super::page("title: sometitle\ndate: 2014-01-01\nthis is a post.\nwith multiple lines!").is_err());
    }

    #[test]
    fn test_page() {
        assert!(super::page("title: sometitle\ndate: 2014-01-01\n\nthis is a post.").is_ok());

        let mut header = HashMap::new();
        header.insert("title".to_string(), "sometitle".to_string());
        header.insert("date".to_string(), "2014-01-01".to_string());
        let header = header;

        let post = Page::new(header, "this is a post.\nwith multiple lines!");

        assert_eq!(super::page("title: sometitle\ndate: 2014-01-01\n\nthis is a post.\nwith multiple lines!"), Ok(page));
    }
}
