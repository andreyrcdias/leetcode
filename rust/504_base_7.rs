fn convert_to_base7(num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    if num < 0 {
        return format!("-{}", convert_to_base7(-num));
    }
    let mut base7_digits = Vec::new();
    let mut n = num;
    while n > 0 {
        base7_digits.push((n % 7).to_string());
        n /= 7;
    }
    base7_digits.reverse();
    base7_digits.join("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_to_base7() {
        assert_eq!(convert_to_base7(100), String::from("202"));
        assert_eq!(convert_to_base7(-7), String::from("-10"));
    }
}
