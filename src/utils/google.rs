extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
          .add(b'<').add(b'>').add(b'`');

pub fn construct_google_query(query : &str ) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let search_url = format!("https://www.google.com/search?q={}", encoded_query);

    return search_url;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_whitespace() {
        let test = "hello";
        assert_eq!(construct_google_query(test), "https://www.google.com/search?q=hello");
    }

    #[test]
    fn test_whitespace() {
        let test = "hello world";
        assert_eq!(construct_google_query(test), "https://www.google.com/search?q=hello%20world");
    }
}