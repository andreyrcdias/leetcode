use std::collections::HashMap;

fn repeated_character(s: String) -> char {
    let mut count = HashMap::new();
    for c in s.chars() {
        let cnt = count.entry(c).or_insert(0);
        *cnt += 1;
        if *cnt == 2 {
            return c;
        }
    }
    '\0'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_repeated_character() {
        assert_eq!(repeated_character("abccbaacz".to_string()), 'c');
        assert_eq!(repeated_character("abcdd".to_string()), 'd');
    }
}
