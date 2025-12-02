fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let k = k as usize;
    let bytes = s.as_bytes();
    let mut result = Vec::new();

    let mut i = 0;
    while i < bytes.len() {
        let end = usize::min(i + k, bytes.len());
        let group = &s[i..end];
        let mut group_str = group.to_string();
        if group_str.len() < k {
            group_str.extend(std::iter::repeat(fill).take(k - group_str.len()));
        }
        result.push(group_str);
        i += k;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_divide_string() {
        assert_eq!(
            divide_string("abcdefghi".to_string(), 3, 'x'),
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
        );
        assert_eq!(
            divide_string("abcdefghij".to_string(), 3, 'x'),
            vec![
                "abc".to_string(),
                "def".to_string(),
                "ghi".to_string(),
                "jxx".to_string()
            ]
        );
    }
}
