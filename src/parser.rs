pub use self::grammar::header;

// grammar.rustpeg contains the parsing expression grammar needed in order to
// parse posts.
peg_file! grammar("grammar.rustpeg")

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_invalid_header() {
        assert!(super::header(": ").is_err());
        assert!(super::header(": \n").is_err());
        assert!(super::header("title: ").is_err());
        assert!(super::header("title:sometitle").is_err());
        assert!(super::header("title: \n").is_err());
    }

    #[test]
    fn test_header() {
        let mut map = HashMap::new();
        map.insert("title", "sometitle");
        map.insert("date", "2014-01-01");
        let map = map;

        assert_eq!(super::header("title: sometitle\ndate: 2014-01-01"), Ok(map));
    }
}
