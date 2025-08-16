fn backspace(s: String) -> Vec<char> {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c != '#' {
            stack.push(c);
        } else if !stack.is_empty() {
            stack.pop();
        }
    }
    stack
}

fn backspace_compare(s: String, t: String) -> bool {
    backspace(s) == backspace(t)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_backspace_compare() {
        assert_eq!(
            backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
        assert_eq!(
            backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
        assert_eq!(backspace_compare("a#c".to_string(), "b".to_string()), false);
    }
}
