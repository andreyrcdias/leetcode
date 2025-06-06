use std::collections::HashMap;

fn find_the_difference(s: String, t: String) -> char {
    let mut char_count = HashMap::new();
    for ch in s.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }
    for ch in t.chars() {
        let count = char_count.entry(ch).or_insert(0);
        if *count == 0 {
            return ch;
        }
        *count -= 1;
    }
    '\0'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(
            find_the_difference(String::from("abcd"), String::from("abcde")),
            'e'
        );
        assert_eq!(
            find_the_difference(String::from(""), String::from("y")),
            'y'
        );
        assert_eq!(
            find_the_difference(String::from("a"), String::from("a")),
            '\u{0000}'
        );
    }
}
