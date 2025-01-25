
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut unique_count = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[unique_count - 1] {
                nums[unique_count] = nums[i];
                unique_count += 1;
            }
        }

        unique_count as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let length = remove_duplicates(&mut nums);

        assert_eq!(length, 5);
        assert_eq!(&nums[..length as usize], &[0, 1, 2, 3, 4]);
        assert_eq!(&nums[length as usize..], &[0, 0, 0, 0, 0]);
    }
}
