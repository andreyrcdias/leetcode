fn is_english_letter(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn reverse_only_letters(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let (mut i, mut j) = (0, chars.len() - 1);

    while i < j {
        while i < j && !is_english_letter(chars[i]) {
            i += 1;
        }
        while i < j && !is_english_letter(chars[j]) {
            j -= 1;
        }
        if i < j {
            chars.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    chars.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_only_letters() {
        assert_eq!(
            reverse_only_letters("ab-cd".to_string()),
            "dc-ba".to_string()
        );
        assert_eq!(
            reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba".to_string()
        );
        assert_eq!(
            reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!".to_string()
        );
    }
}
