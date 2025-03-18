fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for x in nums {
        let mod_value = x % 3;
        if mod_value != 0 {
            ans += mod_value.min(3 - mod_value);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimum_operations() {
        assert_eq!(minimum_operations(vec![1, 2, 3, 4]), 3);
        assert_eq!(minimum_operations(vec![3, 6, 9]), 0);
    }
}
