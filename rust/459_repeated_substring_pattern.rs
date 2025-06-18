fn repeated_substring_pattern(s: String) -> bool {
    let double_s = format!("{}{}", s, s);
    double_s[1..double_s.len() - 1].contains(&s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_repeated_substring_pattern() {
        assert_eq!(repeated_substring_pattern(String::from("abab")), true);
        assert_eq!(repeated_substring_pattern(String::from("aba")), false);
        assert_eq!(
            repeated_substring_pattern(String::from("abcabcabcabc")),
            true
        );
    }
}
