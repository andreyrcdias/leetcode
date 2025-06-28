fn reverse_str(s: String, k: i32) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let size = chars.len();
    for i in (0..size).step_by((2 * k) as usize) {
        let end = std::cmp::min(i + k as usize, size);
        chars[i..end].reverse();
    }
    chars.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_str() {
        assert_eq!(reverse_str("abcdefg".to_string(), 2), "bacdfeg".to_string());
        assert_eq!(reverse_str("abcd".to_string(), 2), "bacd".to_string());
    }
}
