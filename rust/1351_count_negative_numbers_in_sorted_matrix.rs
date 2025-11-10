fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut negatives = 0;
    for row in grid {
        for num in row {
            if num < 0 {
                negatives += 1;
            }
        }
    }
    negatives
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_negatives() {
        assert_eq!(
            count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3],
            ]),
            8
        );
        assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }
}
