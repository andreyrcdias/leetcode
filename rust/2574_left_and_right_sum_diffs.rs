fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let mut right_sum: i32 = nums.iter().sum();
    let mut left_sum: i32 = 0;

    let mut result = Vec::with_capacity(nums.len());
    for num in nums {
        right_sum -= num;
        result.push(abs_diff(right_sum, left_sum));
        left_sum += num;
    }
    result
}

fn abs_diff(a: i32, b: i32) -> i32 {
    let diff = a - b;
    if diff < 0 {
        -diff
    } else {
        diff
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_left_right_difference() {
        assert_eq!(
            left_right_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
        assert_eq!(left_right_difference(vec![1]), vec![0]);
    }
}
