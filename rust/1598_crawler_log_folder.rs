fn min_operations(logs: Vec<String>) -> i32 {
    let mut curr_depth: i32 = 0;

    for operation in logs {
        match operation.as_str() {
            "../" => {
                // move to parent dir
                if curr_depth > 0 {
                    curr_depth -= 1;
                }
            }
            "./" => {
                // stay current dir
                continue;
            }
            _ => {
                // move into subdir
                curr_depth += 1;
            }
        }
    }
    curr_depth
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(
            min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "../".to_string(),
                "d21/".to_string(),
                "./".to_string()
            ]),
            2
        );
        assert_eq!(
            min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "./".to_string(),
                "d3/".to_string(),
                "../".to_string(),
                "d31/".to_string()
            ]),
            3
        );
        assert_eq!(
            min_operations(vec![
                "d1/".to_string(),
                "../".to_string(),
                "../".to_string(),
                "../".to_string(),
            ]),
            0 // failes, return -2
        );
    }
}
