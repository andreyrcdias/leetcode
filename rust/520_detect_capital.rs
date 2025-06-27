fn detect_capital_use(word: String) -> bool {
    let mut capital = 0;
    for ch in word.chars() {
        if ch.is_uppercase() {
            capital += 1;
        }
    }
    capital == 0
        || capital == word.len()
        || (capital == 1 && word.chars().next().unwrap().is_uppercase())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_capital_use() {
        assert_eq!(detect_capital_use("USA".to_string()), true);
        assert_eq!(detect_capital_use("FlaG".to_string()), false);
        assert_eq!(
            detect_capital_use("ffffffffffffffffffffF".to_string()),
            false
        );
    }
}
