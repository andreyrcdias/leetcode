fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut richests: Vec<i32> = Vec::new();
    for account in accounts {
        let sum = account.iter().sum();
        richests.push(sum);
    }
    *richests.iter().max().unwrap_or(&0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximum_wealth() {
        assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1],]), 6);
        assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
        assert_eq!(
            maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17
        );
    }
}
