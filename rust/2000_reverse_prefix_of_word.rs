fn reverse_prefix(word: String, ch: char) -> String {
    let mut first_occurrence_index = None;
    for (i, c) in word.chars().enumerate() {
        if c == ch {
            first_occurrence_index = Some(i);
            break;
        }
    }
    let index = match first_occurrence_index {
        Some(i) => i,
        None => return word,
    };
    let (prefix, suffix) = word.split_at(index + 1);
    let reversed_prefix: String = prefix.chars().rev().collect();
    format!("{}{}", reversed_prefix, suffix)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_prefix() {
        assert_eq!(
            reverse_prefix(String::from("abcdefd"), 'd'),
            String::from("dcbaefd")
        );
        assert_eq!(
            reverse_prefix(String::from("xyxzxe"), 'z'),
            String::from("zxyxxe")
        );
        assert_eq!(
            reverse_prefix(String::from("abcd"), 'z'),
            String::from("abcd")
        );
    }
}
