use std::collections::HashMap;

fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut sneaky_nums: HashMap<i32, usize> = HashMap::new();
    for &num in nums.iter() {
        *sneaky_nums.entry(num).or_insert(0) += 1;
    }
    let mut result: Vec<i32> = sneaky_nums
        .iter()
        .filter_map(|(&num, &count)| if count > 1 { Some(num) } else { None })
        .collect();

    result.sort();

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_sneaky_numbers() {
        assert_eq!(get_sneaky_numbers(vec![0, 1, 1, 0]), vec![0, 1]);
        assert_eq!(get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2]), vec![2, 3]);
        assert_eq!(
            get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]),
            vec![4, 5]
        );
    }
}
