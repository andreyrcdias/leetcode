fn min_element(nums: Vec<i32>) -> i32 {
    nums.iter()
        .map(|&num| {
            num.abs()
                .to_string()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .sum::<i32>()
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_element() {
        assert_eq!(min_element(vec![10, 12, 13, 14]), 1);
        assert_eq!(min_element(vec![1, 2, 3, 4]), 1);
        assert_eq!(min_element(vec![999, 19, 199]), 10);
    }
}
