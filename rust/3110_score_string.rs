fn score_of_string(s: String) -> i32 {
    let mut score: i32 = 0;
    for i in 0..s.len() - 1 {
        let a = s.chars().nth(i).unwrap();
        let b = s.chars().nth(i + 1).unwrap();
        score += (a as i32 - b as i32).abs();
    }
    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score_of_string() {
        assert_eq!(score_of_string(String::from("hello")), 13);
        assert_eq!(score_of_string(String::from("zaz")), 50);
    }
}
