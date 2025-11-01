fn max_depth(s: &str) -> i32 {
    let (mut max_depth, mut current_depth) = (0, 0);

    for char in s.chars() {
        if char == '(' {
            current_depth += 1;
            max_depth = max_depth.max(current_depth);
        } else if char == ')' {
            current_depth -= 1;
        }
    }
    max_depth
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_depth() {
        assert_eq!(max_depth("(1+(2*3)+((8)/4))+1"), 3);
        assert_eq!(max_depth("(1)+((2))+(((3)))"), 3);
        assert_eq!(max_depth("()(())((()()))"), 3);
    }
}
