fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last_non_zero_found_index = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(last_non_zero_found_index, i);
            last_non_zero_found_index += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
        let expected: Vec<i32> = vec![1, 3, 12, 0, 0];
        move_zeroes(&mut nums);
        assert_eq!(nums, expected);

        let mut nums: Vec<i32> = vec![0];
        let expected: Vec<i32> = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }
}
