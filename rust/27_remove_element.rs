fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&num| num != val);
    nums.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut vec1 = vec![3, 2, 2, 3];
        assert_eq!(remove_element(&mut vec1, 3), 2);
        let mut vec2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut vec2, 2), 5);
    }
}
