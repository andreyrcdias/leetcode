fn sum_base(n: i32, k: i32) -> i32 {
    let mut x = n;
    let mut digit_sum = 0;
    while x > 0 {
        digit_sum += x % k;
        x /= k;
    }
    digit_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_base() {
        assert_eq!(sum_base(34, 6), 9);
        assert_eq!(sum_base(10, 10), 1);
    }
}
