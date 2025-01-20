fn defang_i_paddr(address: String) -> String {
    let mut defanged_ips: Vec<char> = Vec::new();
    for ch in address.chars() {
        if ch == '.' {
            defanged_ips.extend_from_slice(&['[', ch, ']']);
        } else {
            defanged_ips.push(ch);
        }
    }
    defanged_ips.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        assert_eq!(
            defang_i_paddr(String::from("1.1.1.1")),
            String::from("1[.]1[.]1[.]1")
        );
        assert_eq!(
            defang_i_paddr(String::from("255.100.50.0")),
            String::from("255[.]100[.]50[.]0")
        );
    }
}
