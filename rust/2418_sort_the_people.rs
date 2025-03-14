fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut idxs: Vec<usize> = (0..heights.len()).collect();
    idxs.sort_by(|&a, &b| heights[b].cmp(&heights[a]));
    idxs.iter().map(|&i| names[i].clone()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_people() {
        assert_eq!(
            sort_people(
                vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
                vec![180, 165, 170]
            ),
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        );
        assert_eq!(
            sort_people(
                vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
                vec![155, 185, 150]
            ),
            vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()]
        );
    }
}
