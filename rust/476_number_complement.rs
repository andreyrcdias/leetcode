fn find_complement(num: i32) -> i32 {
    let bin_num = format!("{:b}", num);
    let parts: Vec<char> = bin_num
        .chars()
        .map(|ch| if ch == '1' { '0' } else { '1' })
        .collect();
    let complement: String = parts.iter().collect();
    i32::from_str_radix(&complement, 2).unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_complement() {
        assert_eq!(find_complement(5), 2);
        assert_eq!(find_complement(1), 0);
    }
}
