fn make_fancy_string(s: String) -> String {
    let mut result = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        if i < 2 || bytes[i] != bytes[i - 1] || bytes[i] != bytes[i - 2] {
            result.push(bytes[i] as char);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_fancy_string() {
        assert_eq!(
            make_fancy_string("leeetcode".to_string()),
            "leetcode".to_string()
        );
        assert_eq!(
            make_fancy_string("aaabaaaa".to_string()),
            "aabaa".to_string()
        );
        assert_eq!(make_fancy_string("aab".to_string()), "aab".to_string());
    }
}
