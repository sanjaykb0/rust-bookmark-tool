pub mod google;
pub mod twitter;
pub mod github;

pub fn get_str_query(t : &str) -> &str{
    if t.contains(" ") {
        let query_index = t.find(' ').unwrap_or(0);
        return &t[..query_index];
    }
    return &t;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_str_query_nowhitespace() {
        let result = get_str_query("tw");
        let expected = "tw";
        assert_eq!(result, expected);
    }

    #[test]
    fn get_str_query_whitespace() {
        let result = get_str_query("tw Hello");
        let expected = "tw";
        assert_eq!(result, expected);
    }
}