fn missing_number(nums: Vec<i32>) -> i32 {
    let mut ans = nums.len() as i32;
    for (i, &num) in nums.iter().enumerate() {
        ans ^= i as i32 ^ num;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_missing_number() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
        assert_eq!(missing_number(vec![0, 1]), 2);
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
