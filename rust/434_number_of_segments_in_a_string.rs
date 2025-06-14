fn count_segments(s: String) -> i32 {
    let parts: Vec<&str> = s.split_whitespace().collect();
    parts.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_segments() {
        assert_eq!(count_segments(String::from("Hello, my name is John")), 5);
        assert_eq!(count_segments(String::from("Hello")), 1);
    }
}
