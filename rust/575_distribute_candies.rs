use std::collections::HashSet;

fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let unique_candies: HashSet<_> = candy_type.iter().collect();
    let half_candies = (candy_type.len() / 2) as i32;
    let size_candies = unique_candies.len() as i32;
    std::cmp::min(half_candies, size_candies)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distribute_candies() {
        assert_eq!(distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(distribute_candies(vec![1, 1, 2, 3]), 2);
        assert_eq!(distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
