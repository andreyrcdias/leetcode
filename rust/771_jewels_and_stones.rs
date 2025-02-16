use std::collections::HashSet;

fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let jewels_set: HashSet<char> = jewels.chars().collect();
    let count = stones.chars().filter(|&c| jewels_set.contains(&c)).count();
    count as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_jewels_in_stones() {
        assert_eq!(
            num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb")),
            3
        );
        assert_eq!(
            num_jewels_in_stones(String::from("z"), String::from("ZZ")),
            0
        );
    }
}
