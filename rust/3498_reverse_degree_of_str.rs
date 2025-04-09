fn reverse_degree(s: String) -> i32 {
    let mut total = 0;
    for (i, ch) in s.chars().enumerate() {
        if ch.is_alphabetic() {
            let rev = 27 - (ch.to_ascii_lowercase() as usize - 'a' as usize + 1);
            total += rev as i32 * (i as i32 + 1);
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_degree() {
        assert_eq!(reverse_degree("abc".to_string()), 148);
        assert_eq!(reverse_degree("zaza".to_string()), 160);
    }
}
