fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for num in nums {
        for x in num.to_string().chars() {
            let digit = x.to_digit(10).unwrap() as i32;
            result.push(digit);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_separate_digits() {
        assert_eq!(
            separate_digits(vec![13, 25, 83, 77]),
            vec![1, 3, 2, 5, 8, 3, 7, 7]
        );
        assert_eq!(separate_digits(vec![7, 1, 3, 9]), vec![7, 1, 3, 9]);
    }
}
