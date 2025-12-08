fn add_digits(num: i32) -> i32 {
    if num == 0 {
        0
    } else {
        (num - 1).rem_euclid(9) + 1
    }
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
