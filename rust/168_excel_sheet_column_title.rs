fn convert_to_title(column_number: i32) -> String {
    let mut number = column_number;
    let mut title = String::new();

    while number > 0 {
        number -= 1;
        let remainder = number % 26;
        title.push((b'A' + remainder as u8) as char);
        number /= 26;
    }
    title.chars().rev().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_to_title() {
        assert_eq!(convert_to_title(1), "A".to_string());
        assert_eq!(convert_to_title(28), "AB".to_string());
        assert_eq!(convert_to_title(701), "ZY".to_string());
    }
}
