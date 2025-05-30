fn count_bits(n: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..=n {
        let bin_repr = format!("{:b}", i);
        let ones = bin_repr.chars().filter(|&c| c == '1').count() as i32;
        ans.push(ones);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_sum_query() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
