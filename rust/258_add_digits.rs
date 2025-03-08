fn add_digits(num: i32) -> i32 {
    let mut n = num;
    while n >= 10 {
        n = n
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i32)
            .sum();
    }
    n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_digits() {
        assert_eq!(add_digits(38), 2);
        assert_eq!(add_digits(0), 0);
    }
}
