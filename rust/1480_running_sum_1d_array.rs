fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::with_capacity(nums.len());
    let mut cumulative_sum = 0;
    for num in nums {
        cumulative_sum += num;
        out.push(cumulative_sum);
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
        assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
    }
}
