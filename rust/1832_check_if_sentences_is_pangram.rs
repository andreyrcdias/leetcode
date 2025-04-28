use std::collections::HashMap;

fn check_if_pangram(sentence: String) -> bool {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for c in sentence.chars() {
        *char_count.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }
    let all_letters_present = ('a'..='z').all(|c| char_count.contains_key(&c));
    all_letters_present
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_if_pangram() {
        assert_eq!(
            check_if_pangram(String::from("thequickbrownfoxjumpsoverthelazydog")),
            true
        );
        assert_eq!(check_if_pangram(String::from("leetcode")), false);
    }
}
