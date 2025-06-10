use std::collections::HashMap;

fn longest_palindrome(s: String) -> i32 {
    let mut count = HashMap::new();
    let mut ans = 0;

    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    for &c in count.values() {
        ans += if c % 2 == 0 { c } else { c - 1 };
    }

    let has_odd_count = count.values().any(|&c| c % 2 == 1);
    ans + if has_odd_count { 1 } else { 0 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome(String::from("abccccdd")), 7);
        assert_eq!(longest_palindrome(String::from("a")), 1);
    }
}
