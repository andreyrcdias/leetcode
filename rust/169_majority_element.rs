fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut candidate, mut count) = (0, 0);
    for num in nums {
        if count == 0 {
            candidate = num;
        }
        count += if num == candidate { 1 } else { -1 };
    }
    candidate
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
