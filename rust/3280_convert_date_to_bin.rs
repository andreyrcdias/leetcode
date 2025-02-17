fn convert_date_to_binary(date: String) -> String {
    let mut bin_parts: Vec<String> = Vec::new();
    for part in date.split("-") {
        let ascii_num: u32 = part.parse().unwrap();
        let bin_str = format!("{:b}", ascii_num);
        bin_parts.push(bin_str);
    }
    bin_parts.join("-")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_date_to_binary() {
        assert_eq!(
            convert_date_to_binary(String::from("2080-02-29")),
            String::from("100000100000-10-11101")
        );
        assert_eq!(
            convert_date_to_binary(String::from("1900-01-01")),
            String::from("11101101100-1-1")
        );
    }
}
