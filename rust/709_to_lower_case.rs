fn to_lower_case(s: String) -> String {
    s.to_lowercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_lower_case() {
        assert_eq!(to_lower_case(String::from("Hello")), String::from("hello"));
        assert_eq!(to_lower_case(String::from("here")), String::from("here"));
        assert_eq!(
            to_lower_case(String::from("LOVELY")),
            String::from("lovely")
        );
    }
}
