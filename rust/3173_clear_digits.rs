fn clear_digits(s: String) -> String {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if c.is_digit(10) {
            if !stack.is_empty() {
                stack.pop();
            }
        } else {
            stack.push(c);
        }
    }
    stack.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clear_digits() {
        assert_eq!(clear_digits(String::from("abc")), String::from("abc"));
        assert_eq!(clear_digits(String::from("cb34")), String::from(""));
    }
}
