fn check_perfect_number(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    let mut sum = 1;
    let limit = (num as f64).sqrt() as i32;
    for i in 2..=limit {
        if num % i == 0 {
            sum += i;
            if i != num / i {
                sum += num / i;
            }
        }
    }
    sum == num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_perfect_number() {
        assert_eq!(check_perfect_number(28), true);
        assert_eq!(check_perfect_number(7), false);
        assert_eq!(check_perfect_number(999999999), false);
    }
}
