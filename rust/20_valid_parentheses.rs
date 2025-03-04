use std::collections::HashMap;

fn is_valid(s: String) -> bool {
    let closing_by_opening: HashMap<char, char> = [('(', ')'), ('{', '}'), ('[', ']')]
        .iter()
        .cloned()
        .collect();
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        if closing_by_opening.contains_key(&ch) {
            stack.push(ch);
        } else {
            if stack.is_empty() || closing_by_opening[&stack.pop().unwrap()] != ch {
                return false;
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
    }
}
