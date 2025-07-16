fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut ns = nums.clone();
    ns.sort();
    let sz = ns.len();
    let opt1 = ns[sz - 1] * ns[0] * ns[1];
    let opt2 = ns[sz - 1] * ns[sz - 2] * ns[sz - 3];
    opt1.max(opt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximum_product() {
        assert_eq!(maximum_product(vec![1, 2, 3]), 6);
        assert_eq!(maximum_product(vec![1, 2, 3, 4]), 24);
        assert_eq!(maximum_product(vec![-1, -2, -3]), -6);
    }
}
