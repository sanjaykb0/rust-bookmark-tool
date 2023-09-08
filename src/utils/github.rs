extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
          .add(b'<').add(b'>').add(b'`');

pub fn GH_construct_github_query(query: &str) -> String {
    if query == "gh" {
        return String::from("https://github.com");

    } else if &query[..5] == "gh r " {
        return GH_construct_repo_query(&query[5..]);

    } else if &query[..5] == "gh p " {
        return GH_construct_profile_query(&query[5..]);
    }

    // if query = "gh <profile_name>"
    GH_construct_profile_query(&query[3..]) 
}

pub fn GH_construct_profile_query(query: &str) -> String {
    let encoded_query : String = utf8_percent_encode(query, FRAGMENT).to_string();
    return format!("https://github.com/{}", encoded_query);
}

pub fn GH_construct_repo_query(query: &str) -> String {
    if query.len() <= 1 {
        return String::from("https://github.com/explore");
    }

    let encoded_query : String = utf8_percent_encode(query, FRAGMENT).to_string();
    return format!("https://github.com/search?q={}&type=repositories", encoded_query);
}


#[cfg(test)]
mod tests {
    use super::*;
        // BASIC QUERY TEST 
    #[test]
    fn TEST_GH_query_blank() {
        let test_query = "gh";
        assert_eq!(GH_construct_github_query(test_query), "https://github.com");
    }

    #[test]
    fn TEST_GH_query_bare_profile() {
        let test_query = "gh sanjaykb0";
        assert_eq!(GH_construct_github_query(test_query), "https://github.com/sanjaykb0");
    }

        // PROFILE QUERY TEST
    #[test]
    fn TEST_GH_query_profile() {
        let test_query = "gh p sanjaykb0";
        assert_eq!(GH_construct_github_query(test_query), "https://github.com/sanjaykb0");
    }

        // REPO TEST FUNCTIONS
    #[test]
    fn TEST_GH_query_repo_empty() {
        let test_query = "gh r ";
        assert_eq!(GH_construct_github_query(test_query), "https://github.com/explore")
    }

    #[test]
    fn TEST_GH_query_repo_no_whitespace() {
        let test_query = "gh r lichess";
        assert_eq!(GH_construct_github_query(test_query), "https://github.com/search?q=lichess&type=repositories")
    }

    #[test]
    fn TEST_GH_query_repo_whitespace() {
        let test_query = "gh r li chess";
        assert_eq!(GH_construct_github_query(test_query), "https://github.com/search?q=li%20chess&type=repositories")
    }

}