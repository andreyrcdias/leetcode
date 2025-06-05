use std::collections::HashMap;

fn is_first_uniq_char(s: String) -> i32 {
    let mut char_map: HashMap<char, i32> = HashMap::new();
    for ch in s.chars() {
        if !char_map.contains_key(&ch) {
            char_map.insert(ch, 1);
        } else {
            *char_map.get_mut(&ch).unwrap() += 1;
        }
    }

    for (i, ch) in s.chars().enumerate() {
        if let Some(&count) = char_map.get(&ch) {
            if count == 1 {
                return i as i32;
            }
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(is_first_uniq_char(String::from("leetcode")), 0);
        assert_eq!(is_first_uniq_char(String::from("loveleetcode")), 2);
        assert_eq!(is_first_uniq_char(String::from("aabb")), -1);
    }
}
