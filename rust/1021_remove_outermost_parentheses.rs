fn remove_outer_parentheses(s: String) -> String {
    let mut answer: Vec<char> = Vec::new();
    let mut opened = 0;
    for ch in s.chars() {
        if ch == '(' {
            opened += 1;
            if opened > 1 {
                answer.push(ch);
            }
        } else {
            opened -= 1;
            if opened > 0 {
                answer.push(ch);
            }
        }
    }
    answer.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_outer_parentheses() {
        assert_eq!(
            remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
        assert_eq!(
            remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
        assert_eq!(remove_outer_parentheses("()()".to_string()), "".to_string());
    }
}
