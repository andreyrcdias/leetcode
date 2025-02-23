fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut concatenated: Vec<i32> = Vec::new();
    for _i in 0..2 {
        for num in &nums {
            concatenated.push(*num);
        }
    }
    concatenated
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_concatenation() {
        assert_eq!(get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
        assert_eq!(
            get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
