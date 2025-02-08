fn hamming_weight(n: i32) -> i32 {
    let mut w: i32 = 0;
    let b = String::from(format!("{n:b}"));
    for ch in b.chars() {
        if ch == '1' {
            w += 1;
        }
    }
    w
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(hamming_weight(11), 3);
        assert_eq!(hamming_weight(128), 1);
        assert_eq!(hamming_weight(2147483645), 30);
    }
}
