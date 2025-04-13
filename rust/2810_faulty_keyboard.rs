fn final_string(s: String) -> String {
    let mut ans: Vec<char> = Vec::new();
    for ch in s.chars() {
        if ch == 'i' {
            ans.reverse();
        } else {
            ans.push(ch);
        }
    }
    ans.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_final_string() {
        assert_eq!(final_string("string".to_string()), "rtsng".to_string());
        assert_eq!(final_string("poiinter".to_string()), "ponter".to_string());
    }
}
