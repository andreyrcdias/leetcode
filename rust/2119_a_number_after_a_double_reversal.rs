fn is_same_after_reversals(num: i32) -> bool {
    num == 0 || num % 10 != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(is_same_after_reversals(526), true);
        assert_eq!(is_same_after_reversals(1800), false);
        assert_eq!(is_same_after_reversals(0), true);
    }
}
