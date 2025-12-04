fn remove_trailing_zeros(num: String) -> String {
    let mut end = num.len();
    let num_bytes = num.as_bytes();
    while end > 0 && num_bytes[end - 1] == b'0' {
        end -= 1;
    }
    num[..end].to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_trailing_zeros() {
        assert_eq!(
            remove_trailing_zeros("51230100".to_string()),
            "512301".to_string()
        );
        assert_eq!(remove_trailing_zeros("123".to_string()), "123".to_string());
    }
}
