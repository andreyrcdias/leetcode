fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut indexes: Vec<i32> = Vec::new();
    for (i, word) in words.iter().enumerate() {
        for ch in word.chars() {
            if ch == x {
                indexes.push(i as i32);
                break;
            }
        }
    }
    indexes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_words_containing() {
        assert_eq!(
            find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e'),
            vec![0, 1]
        );
        assert_eq!(
            find_words_containing(
                vec![
                    "abc".to_string(),
                    "bcd".to_string(),
                    "aaaa".to_string(),
                    "cbc".to_string()
                ],
                'a'
            ),
            vec![0, 2]
        );
        assert_eq!(
            find_words_containing(
                vec![
                    "abc".to_string(),
                    "bcd".to_string(),
                    "aaaa".to_string(),
                    "cbc".to_string()
                ],
                'z'
            ),
            vec![]
        );
    }
}
