fn is_perfect_square(num: i32) -> bool {
    let root = (num as f64).sqrt() as i32;
    root * root == num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(is_perfect_square(16), true);
        assert_eq!(is_perfect_square(14), false);
    }
}
