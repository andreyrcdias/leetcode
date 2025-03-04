fn truncate_sentence(s: String, k: i32) -> String {
    let k = k as usize;
    let words: Vec<&str> = s.split_whitespace().collect();
    let truncated_words = &words[..k.min(words.len())];
    truncated_words.join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_truncate_sentence() {
        assert_eq!(
            truncate_sentence(String::from("Hello how are you Contestant"), 4),
            "Hello how are you"
        );
        assert_eq!(
            truncate_sentence(String::from("What is the solution to this problem"), 4),
            "What is the solution"
        );
        assert_eq!(
            truncate_sentence(String::from("chopper is not a tanuki"), 5),
            "chopper is not a tanuki"
        );
    }
}
