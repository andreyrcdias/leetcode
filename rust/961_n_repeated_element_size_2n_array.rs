fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    for i in 0..(n - 2) {
        if nums[i] == nums[i + 1] || nums[i] == nums[i + 2] {
            return nums[i];
        }
    }
    nums[n - 1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_repeated_n_times() {
        assert_eq!(repeated_n_times(vec![1, 2, 3, 3]), 3);
        assert_eq!(repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
        assert_eq!(repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
