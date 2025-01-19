fn is_palindrom(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        if ch.is_alphanumeric() {
            stack.push(ch.to_ascii_lowercase());
        }
    }

    for ch in s.chars() {
        if let Some(&top_char) = stack.last() {
            if top_char == ch.to_ascii_lowercase() {
                stack.pop();
            }
        }
    }

    stack.len() == 0
}

fn main() {
    let input_string = "A man, a plan, a canal: Panama".to_string();
    // let input_stirng = "race a car".to_string();
    // let input_stirng = " ".to_string();

    println!("Palindrom: {}", is_palindrom(input_string));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            is_palindrom("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(is_palindrom("race a car".to_string()), false);
        assert_eq!(is_palindrom(" ".to_string()), true);
    }
}
