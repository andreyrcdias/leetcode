fn min_partitions(n: String) -> i32 {
    let mut max_digit = 0;
    for ch in n.chars() {
        if let Some(digit) = ch.to_digit(10) {
            if digit > max_digit {
                max_digit = digit;
            }
        }
    }
    max_digit as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_partitions() {
        assert_eq!(min_partitions(String::from("32")), 3);
        assert_eq!(min_partitions(String::from("82734")), 8);
        assert_eq!(min_partitions(String::from("27346209830709182346")), 9);
    }
}
