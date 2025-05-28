fn is_power_of_three(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let mut num = n;
    while num % 3 == 0 {
        num /= 3;
    }
    num == 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_sum_query() {
        assert_eq!(is_power_of_three(27), true);
        assert_eq!(is_power_of_three(0), false);
        assert_eq!(is_power_of_three(-1), false);
    }
}
