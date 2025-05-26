use std::collections::HashMap;

fn word_pattern(pattern: String, s: String) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    if pattern.len() != words.len() {
        return false;
    }

    let mut char_to_word_map = HashMap::new();
    let mut word_to_char_map = HashMap::new();

    for (char, word) in pattern.chars().zip(words.iter()) {
        if let Some(mapped_word) = char_to_word_map.get(&char) {
            if mapped_word != word {
                return false;
            }
        } else {
            char_to_word_map.insert(char, *word);
        }

        if let Some(mapped_char) = word_to_char_map.get(*word) {
            if mapped_char != &char {
                return false;
            }
        } else {
            word_to_char_map.insert(*word, char);
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_pattern() {
        assert_eq!(
            word_pattern(String::from("abba"), String::from("dog cat cat dog")),
            true
        );
        assert_eq!(
            word_pattern(String::from("abba"), String::from("dog cat cat fish")),
            false
        );
        assert_eq!(
            word_pattern(String::from("aaaa"), String::from("dog cat cat dog")),
            false
        );
    }
}
