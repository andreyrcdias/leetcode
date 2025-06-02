use std::collections::HashSet;

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = nums1.into_iter().collect();
    let mut ans = HashSet::new();
    for num in nums2 {
        if set1.contains(&num) {
            ans.insert(num);
        }
    }
    ans.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
        assert_eq!(intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
    }
}
