fn reverse_bits(x: u32) -> u32 {
    let mut r: u32 = 0;
    let mut x = x;
    for _ in 0..32 {
        r <<= 1;
        r |= x & 1;
        x >>= 1;
    }
    r
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(reverse_bits(0b00000010100101000001111010011100), 964176192);
        assert_eq!(reverse_bits(0b11111111111111111111111111111101), 3221225471);
    }
}
