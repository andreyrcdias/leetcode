fn length_of_last_word(s: String) -> i32 {
    let words: Vec<_> = s.split_whitespace().collect();
    let last_word = words.last().unwrap();
    last_word.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word(String::from("Hello World")), 5);
        assert_eq!(
            length_of_last_word(String::from("   fly me   to   the moon  ")),
            4
        );
        assert_eq!(
            length_of_last_word(String::from("luffy is still joyboy")),
            6
        );
    }
}
