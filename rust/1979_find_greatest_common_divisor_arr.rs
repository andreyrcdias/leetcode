fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn find_gcd(nums: Vec<i32>) -> i32 {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    gcd(max, min)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_gcd() {
        assert_eq!(find_gcd(vec![2, 5, 6, 9, 10]), 2);
        assert_eq!(find_gcd(vec![7, 5, 6, 8, 3]), 1);
        assert_eq!(find_gcd(vec![3, 3]), 3);
    }
}
