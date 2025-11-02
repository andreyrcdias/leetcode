use std::collections::HashMap;

fn max_length_between_equal_characters(s: String) -> i32 {
    let mut ans = -1;
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        if !last_seen.contains_key(&c) {
            last_seen.insert(c, i);
        } else {
            ans = ans.max(i as i32 - last_seen[&c] as i32 - 1);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_length_between_equal_characters() {
        assert_eq!(max_length_between_equal_characters("aa".to_string()), 0);
        assert_eq!(max_length_between_equal_characters("abca".to_string()), 2);
        assert_eq!(max_length_between_equal_characters("cbzxy".to_string()), -1);
    }
}
