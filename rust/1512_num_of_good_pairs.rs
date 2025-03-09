fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut identical_pairs_count = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                identical_pairs_count += 1;
            }
        }
    }
    identical_pairs_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
