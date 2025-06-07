fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let mut i = 0;
    for ch in t.chars() {
        if s.chars().nth(i) == Some(ch) {
            i += 1;
            if i == s.len() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(
            is_subsequence(String::from("abc"), String::from("ahbgdc")),
            true
        );
        assert_eq!(
            is_subsequence(String::from("axc"), String::from("ahbgdc")),
            false
        );
    }
}
