use std::collections::HashMap;

fn single_number(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for &num in nums.iter() {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut min_num = 0;
    let mut min_count = i32::MAX;

    for (&key, &value) in &map {
        if value < min_count {
            min_num = key;
            min_count = value;
        }
    }
    min_num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(single_number(vec![1]), 1);
    }
}
