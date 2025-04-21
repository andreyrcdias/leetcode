fn ugly_number(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let mut num = n;
    for &prime in &[2, 3, 5] {
        while num % prime == 0 {
            num /= prime;
        }
    }
    num == 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ugly_number() {
        assert_eq!(ugly_number(6), true);
        assert_eq!(ugly_number(1), true);
        assert_eq!(ugly_number(14), false);
    }
}
