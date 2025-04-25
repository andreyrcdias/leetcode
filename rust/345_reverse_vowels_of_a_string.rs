fn is_vowel(c: char) -> bool {
    let c = c.to_ascii_lowercase();
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn reverse_vowels(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let (mut i, mut j) = (0, chars.len() - 1);

    while i < j {
        while i < j && !is_vowel(chars[i]) {
            i += 1;
        }
        while i < j && !is_vowel(chars[j]) {
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
    fn test_reverse_vowels() {
        assert_eq!(
            reverse_vowels("IceCreAm".to_string()),
            "AceCreIm".to_string()
        );
        assert_eq!(
            reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }
}
