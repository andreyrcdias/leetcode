struct NumArray {
    cumulative_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut cumulative_sum = vec![0];
        let mut total = 0;
        for &num in &nums {
            total += num;
            cumulative_sum.push(total);
        }
        NumArray { cumulative_sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.cumulative_sum[(right + 1) as usize] - self.cumulative_sum[left as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_sum_query() {
        let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(num_array.sum_range(0, 2), 1);
        assert_eq!(num_array.sum_range(2, 5), -1);
        assert_eq!(num_array.sum_range(0, 5), -3);
    }
}
