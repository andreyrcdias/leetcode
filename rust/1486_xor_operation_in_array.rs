fn xor_operation(n: i32, start: i32) -> i32 {
    let xor_arr: Vec<i32> = (0..n).map(|i| start + 2 * i).collect();
    xor_arr.iter().fold(0, |acc, value| acc ^ value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xor_operation() {
        assert_eq!(xor_operation(5, 0), 8);
        assert_eq!(xor_operation(4, 3), 8);
    }
}
