use std::collections::HashMap;

fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut counter = HashMap::new();
    for &num in &nums1 {
        *counter.entry(num).or_insert(0) += 1;
    }

    let mut ans = Vec::new();
    for &num in &nums2 {
        if let Some(count) = counter.get_mut(&num) {
            if *count > 0 {
                ans.push(num);
                *count -= 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
        assert_eq!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![9, 4]);
    }
}
