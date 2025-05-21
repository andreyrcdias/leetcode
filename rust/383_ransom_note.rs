use std::collections::HashMap;

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut ransom_count = HashMap::new();
    let mut magazine_count = HashMap::new();

    for c in ransom_note.chars() {
        *ransom_count.entry(c).or_insert(0) += 1;
    }

    for c in magazine.chars() {
        *magazine_count.entry(c).or_insert(0) += 1;
    }

    for (c, &count) in ransom_count.iter() {
        if count > *magazine_count.get(c).unwrap_or(&0) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_construct() {
        assert_eq!(can_construct("a".to_string(), "b".to_string()), false);
        assert_eq!(can_construct("aa".to_string(), "ab".to_string()), false);
        assert_eq!(can_construct("aa".to_string(), "aab".to_string()), true);
    }
}
