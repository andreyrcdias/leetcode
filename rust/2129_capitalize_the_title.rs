fn capitalize_title(title: String) -> String {
    let words: Vec<&str> = title.split_whitespace().collect();
    let capitalized_words: Vec<String> = words
        .iter()
        .map(|&word| {
            if word.len() < 3 {
                word.to_lowercase()
            } else {
                let mut chars = word.chars();
                let first_char = chars.next().unwrap().to_uppercase().to_string();
                let rest = chars.collect::<String>().to_lowercase();
                first_char + &rest
            }
        })
        .collect();
    capitalized_words.join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_capitalize_title() {
        assert_eq!(
            capitalize_title("capiTalIze tHe titLe".to_string()),
            "Capitalize The Title".to_string()
        );
        assert_eq!(
            capitalize_title("First leTTeR of EACH Word".to_string()),
            "First Letter of Each Word".to_string()
        );
        assert_eq!(
            capitalize_title("i lOve leetcode".to_string()),
            "i Love Leetcode".to_string()
        );
    }
}
