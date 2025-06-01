fn reverse_string(s: &mut Vec<char>) {
    s.reverse();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_string_case_1() {
        let mut s: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test_reverse_string_case_2() {
        let mut s: Vec<char> = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
