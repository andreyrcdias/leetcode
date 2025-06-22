fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let (mut ones, mut ans) = (0, 0);
    for num in nums {
        if num == 0 {
            ones = 0;
        } else {
            ones += 1;
            ans = ans.max(ones);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_max_consecutive_ones() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}
