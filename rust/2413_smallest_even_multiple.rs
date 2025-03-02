fn smallest_even_multiple(n: i32) -> i32 {
    if n % 2 == 0 {
        n
    } else {
        n * 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smallest_even_multiple() {
        assert_eq!(smallest_even_multiple(5), 10);
        assert_eq!(smallest_even_multiple(6), 6);
    }
}
