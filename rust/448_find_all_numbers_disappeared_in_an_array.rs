fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    let mut seen = vec![false; n as usize + 1];
    for num in nums {
        if num > 0 && num <= n {
            seen[num as usize] = true;
        }
    }
    let disappeared_numbers: Vec<i32> = (1..=n).filter(|&x| !seen[x as usize]).collect();
    disappeared_numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers() {
        assert_eq!(
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
        assert_eq!(find_disappeared_numbers(vec![1, 1]), vec![2]);
        assert_eq!(find_disappeared_numbers(vec![2, 2]), vec![1]);
    }
}
