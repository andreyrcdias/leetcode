fn remove_zeros(n: i64) -> i64 {
    let n_str = format!("{}", n);
    let mut v = Vec::new();
    for ch in n_str.chars() {
        if ch != '0' {
            v.push(ch);
        }
    }
    let result_str: String = v.iter().collect();
    result_str.parse::<i64>().unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_zeros() {
        assert_eq!(remove_zeros(1020030), 123);
        assert_eq!(remove_zeros(1), 1);
    }
}
