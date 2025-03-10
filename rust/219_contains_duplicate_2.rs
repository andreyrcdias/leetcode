use std::collections::HashMap;

fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut seen = HashMap::new();
    for (i, x) in nums.iter().enumerate() {
        if let Some(&last_index) = seen.get(&x) {
            if i - last_index <= k as usize {
                return true;
            }
        }
        seen.insert(x, i);
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
}
