fn largest_odd_number(num: String) -> String {
    for i in (0..num.len()).rev() {
        if (num.as_bytes()[i] - b'0') & 1 == 1 {
            return num[..=i].to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_odd_number() {
        assert_eq!(largest_odd_number("52".to_string()), "5".to_string());
        assert_eq!(largest_odd_number("4206".to_string()), "".to_string());
        assert_eq!(largest_odd_number("35427".to_string()), "35427".to_string());
    }
}
