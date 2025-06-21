fn license_key_formatting(s: String, k: i32) -> String {
    let mut ans: Vec<char> = Vec::new();
    let mut size = 0;
    let ns = s.replace('-', "").to_uppercase();
    for ch in ns.chars().rev() {
        if size > 0 && size % k == 0 {
            ans.push('-');
        }
        ans.push(ch);
        size += 1;
    }
    ans.reverse();
    ans.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_license_key_formatting() {
        assert_eq!(
            license_key_formatting(String::from("5F3Z-2e-9-w"), 4),
            String::from("5F3Z-2E9W")
        );
        assert_eq!(
            license_key_formatting(String::from("2-5g-3-J"), 2),
            String::from("2-5G-3J")
        );
    }
}
