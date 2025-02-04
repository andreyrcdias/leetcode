fn rotate_string(s: String, goal: String) -> bool {
    s.len() == goal.len() && (s.clone() + &s).contains(&goal)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_string() {
        assert_eq!(
            rotate_string(String::from("abcde"), String::from("cdeab")),
            true
        );
        assert_eq!(
            rotate_string(String::from("abcde"), String::from("abced")),
            false
        );
    }
}
