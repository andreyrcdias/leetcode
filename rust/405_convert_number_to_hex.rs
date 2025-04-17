fn to_hex(num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let chars = "0123456789abcdef";
    let mut sb = String::new();
    for i in (0..8).rev() {
        let x = (num >> (4 * i)) & 0xF;
        if !sb.is_empty() || x != 0 {
            sb.push(chars.chars().nth(x as usize).unwrap());
        }
    }
    sb
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_hex() {
        assert_eq!(to_hex(26), "1a".to_string());
        assert_eq!(to_hex(-1), "ffffffff".to_string());
    }
}
