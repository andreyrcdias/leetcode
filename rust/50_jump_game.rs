fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut goal = n - 1;
    for i in (0..n).rev() {
        let max_jump = nums[i] as usize;
        if i + max_jump >= goal {
            goal = i;
        }
    }
    goal == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_jump() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
