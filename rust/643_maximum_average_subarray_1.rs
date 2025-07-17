fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut current_sum: i32 = nums.iter().take(k as usize).sum();
    let mut max_sum = current_sum;
    for i in (k as usize)..nums.len() {
        let new_sum = current_sum + nums[i] - nums[i - k as usize];
        max_sum = std::cmp::max(max_sum, new_sum);
        current_sum = new_sum;
    }
    max_sum as f64 / k as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_max_average() {
        assert_eq!(find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.7500);
        assert_eq!(find_max_average(vec![5], 1), 5.00000);
    }
}
