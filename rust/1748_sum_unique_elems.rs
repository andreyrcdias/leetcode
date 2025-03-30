use std::collections::HashMap;

fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut cnt = HashMap::new();
    for num in nums {
        *cnt.entry(num).or_insert(0) += 1;
    }
    cnt.iter()
        .filter_map(|(&x, &v)| if v == 1 { Some(x) } else { None })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_unique() {
        assert_eq!(sum_of_unique(vec![1, 2, 3, 2]), 4);
        assert_eq!(sum_of_unique(vec![1, 1, 1, 1]), 0);
        assert_eq!(sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
    }
}
