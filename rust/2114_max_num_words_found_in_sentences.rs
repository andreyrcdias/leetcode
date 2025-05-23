fn most_word_found(sentences: Vec<String>) -> i32 {
    let mut max_words: i32 = 0;
    for sentence in sentences {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        let words_len = words.len() as i32;
        if words_len > max_words {
            max_words = words_len;
        }
    }
    max_words
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(
            most_word_found(vec![
                "alice and bob love leetcode".to_string(),
                "i think so too".to_string(),
                "this is great thanks very much".to_string(),
            ]),
            6
        );
        assert_eq!(
            most_word_found(vec![
                "please wait".to_string(),
                "continue to fight".to_string(),
                "continue to win".to_string(),
            ]),
            3
        );
    }
}
