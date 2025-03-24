fn reverse_words(s: String) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    let reversed_words: Vec<String> = words
        .iter()
        .map(|&word| word.chars().rev().collect())
        .collect();
    reversed_words.join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
        assert_eq!(reverse_words("Mr Ding".to_string()), "rM gniD".to_string());
    }
}
