fn sum_of_multiplies(n: i32) -> i32 {
    let mut total_sum = 0;
    let divisors = [3, 5, 7];

    for i in 1..=n {
        if divisors.iter().any(|&d| i % d == 0) {
            total_sum += i;
        }
    }
    total_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_multiplies() {
        assert_eq!(sum_of_multiplies(7), 21);
        assert_eq!(sum_of_multiplies(10), 40);
        assert_eq!(sum_of_multiplies(9), 30);
        assert_eq!(sum_of_multiplies(1), 0);
        assert_eq!(sum_of_multiplies(2), 0);
        assert_eq!(sum_of_multiplies(4), 3);
    }
}
