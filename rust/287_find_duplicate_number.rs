use std::collections::HashSet;

fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut seen = HashSet::new();
    for num in nums {
        if !seen.insert(num) {
            return num;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(find_duplicate(vec![3, 3, 3, 3, 3]), 3);
    }
}
