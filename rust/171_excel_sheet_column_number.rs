fn title_to_number(column_title: String) -> i32 {
    let mut number = 0;
    for ch in column_title.chars() {
        let value = (ch as u8 - b'A' + 1) as i32;
        number = number * 26 + value;
    }
    number
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_title_to_number() {
        assert_eq!(title_to_number("A".to_string()), 1);
        assert_eq!(title_to_number("AB".to_string()), 28);
        assert_eq!(title_to_number("ZY".to_string()), 701);
    }
}
