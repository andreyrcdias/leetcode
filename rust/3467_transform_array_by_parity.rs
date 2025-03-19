fn transform_array(nums: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = nums
        .iter()
        .map(|&num| if num % 2 == 0 { 0 } else { 1 })
        .collect();
    out.sort();
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transform_array() {
        assert_eq!(transform_array(vec![4, 3, 2, 1]), vec![0, 0, 1, 1]);
        assert_eq!(transform_array(vec![1, 5, 1, 4, 2]), vec![0, 0, 1, 1, 1]);
    }
}
