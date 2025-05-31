fn is_power_of_four(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let mut num = n;
    while num % 4 == 0 {
        num /= 4;
    }
    num == 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_sum_query() {
        assert_eq!(is_power_of_four(16), true);
        assert_eq!(is_power_of_four(5), false);
        assert_eq!(is_power_of_four(1), true);
    }
}
