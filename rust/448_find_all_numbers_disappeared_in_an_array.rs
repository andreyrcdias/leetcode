fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    let m = nums.iter().cloned().min().unwrap_or(1);
    let nums_range: Vec<i32> = (m..=n).map(|x| x as i32).collect();
    let disappeared_numbers: Vec<i32> = nums_range
        .into_iter()
        .filter(|&x| !nums.contains(&x))
        .collect();
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
    }
}
