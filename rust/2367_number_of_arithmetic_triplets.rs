fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let mut triplets_count = 0;
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            for k in (j + 1)..nums.len() {
                if nums[j] - nums[i] == diff && nums[k] - nums[j] == diff {
                    triplets_count += 1;
                }
            }
        }
    }
    triplets_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arithmetic_triplets() {
        assert_eq!(arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
        assert_eq!(arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }
}
