fn count_digits(num: i32) -> i32 {
    let mut ans = 0;
    for ch in num.to_string().chars() {
        let val = ch.to_digit(10).unwrap() as i32;
        if val != 0 && num % val == 0 {
            ans += 1;
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(7), 1);
        assert_eq!(count_digits(121), 2);
        assert_eq!(count_digits(1248), 4);
    }
}
