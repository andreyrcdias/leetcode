fn is_power_of_two(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert_eq!(is_power_of_two(1), true);
        assert_eq!(is_power_of_two(16), true);
        assert_eq!(is_power_of_two(3), false);
    }
}
