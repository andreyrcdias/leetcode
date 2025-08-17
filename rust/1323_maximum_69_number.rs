fn maximum69_number(num: i32) -> i32 {
    let modified_num = num.to_string().replacen('6', "9", 1);
    modified_num.parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximum69_number() {
        assert_eq!(maximum69_number(9669), 9969);
        assert_eq!(maximum69_number(9969), 9999);
        assert_eq!(maximum69_number(9999), 9999);
    }
}
