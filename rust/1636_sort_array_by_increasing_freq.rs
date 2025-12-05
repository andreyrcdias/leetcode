use std::collections::HashMap;

fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for &num in &nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut sorted = nums.clone();
    sorted.sort_by(|a, b| {
        let fa = freq.get(a).copied().unwrap_or(0);
        let fb = freq.get(b).copied().unwrap_or(0);
        match fa.cmp(&fb) {
            std::cmp::Ordering::Equal => b.cmp(a),
            other => other,
        }
    });
    sorted
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        assert_eq!(
            frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        );
        assert_eq!(frequency_sort(vec![2, 3, 1, 3, 2]), vec![1, 3, 3, 2, 2]);
        assert_eq!(
            frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
