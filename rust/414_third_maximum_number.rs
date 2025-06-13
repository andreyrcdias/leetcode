use std::collections::HashSet;

fn third_max(nums: Vec<i32>) -> i32 {
    let mut maximums: HashSet<i32> = nums.into_iter().collect();
    if maximums.len() < 3 {
        return *maximums.iter().max().unwrap();
    }
    let mut max = 0;
    for _ in 0..3 {
        if let Some(&max_value) = maximums.iter().max() {
            max = max_value;
            maximums.remove(&max_value);
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_third_max() {
        assert_eq!(third_max(vec![3, 2, 1]), 1);
        assert_eq!(third_max(vec![1, 2]), 2);
        assert_eq!(third_max(vec![2, 2, 3, 1]), 1);
    }
}
