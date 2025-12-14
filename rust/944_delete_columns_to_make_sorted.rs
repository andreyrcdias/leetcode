fn min_deletion_size(strs: Vec<String>) -> i32 {
    if strs.is_empty() {
        return 0;
    }
    let n_rows = strs.len();
    let n_cols = strs[0].len();
    let mut ans: i32 = 0;

    for j in 0..n_cols {
        for i in 0..(n_rows - 1) {
            if strs[i].as_bytes()[j] > strs[i + 1].as_bytes()[j] {
                ans += 1;
                break;
            }
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_deletion_size() {
        assert_eq!(
            min_deletion_size(vec![
                "cba".to_string(),
                "daf".to_string(),
                "ghi".to_string()
            ]),
            1
        );
        assert_eq!(min_deletion_size(vec!["a".to_string(), "b".to_string()]), 0);
        assert_eq!(
            min_deletion_size(vec![
                "xyz".to_string(),
                "wvu".to_string(),
                "tsr".to_string()
            ]),
            3
        );
    }
}
