fn add_binary(a: String, b: String) -> String {
    let aa = i32::from_str_radix(&a, 2).unwrap();
    let bb = i32::from_str_radix(&b, 2).unwrap();
    let sum = aa + bb;
    format!("{:b}", sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(
            add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
