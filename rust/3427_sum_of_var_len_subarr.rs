fn subarray_sum(nums: Vec<i32>) -> i32 {
    let mut total_sum = 0;
    for i in 0..nums.len() {
        let start = (i as i32 - nums[i]).max(0) as usize;
        for j in start..=i {
            total_sum += nums[j];
        }
    }
    total_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subarray_sum() {
        assert_eq!(subarray_sum(vec![2, 3, 1]), 11);
        assert_eq!(subarray_sum(vec![3, 1, 1, 2]), 13);
    }
}
