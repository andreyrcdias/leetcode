fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut aux = Vec::with_capacity((m + n) as usize);
    for i in 0..m {
        aux.push(nums1[i as usize]);
    }
    for i in 0..n {
        aux.push(nums2[i as usize]);
    }
    aux.sort();
    nums1.clear();
    nums1.extend(aux);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let (m, n) = (3, 3);
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_merge_case_2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        let (m, n) = (1, 0);
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_merge_case_3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let (m, n) = (0, 1);
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }
}
