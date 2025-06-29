fn is_balanced(num: String) -> bool {
    let mut even_sum: i32 = 0;
    let mut odd_sum: i32 = 0;
    for i in 0..num.len() {
        let ch = num.chars().nth(i).unwrap();
        if let Some(digit) = ch.to_digit(10) {
            if i % 2 == 0 {
                even_sum += digit as i32;
            } else {
                odd_sum += digit as i32;
            }
        }
    }
    even_sum == odd_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_balanced() {
        assert_eq!(is_balanced(String::from("1234")), false);
        assert_eq!(is_balanced(String::from("24123")), true);
    }
}
