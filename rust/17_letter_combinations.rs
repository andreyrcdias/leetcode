use std::collections::HashMap;

fn letter_combinations(digits: String) -> Vec<String> {
    let phone_map: HashMap<i32, Vec<char>> = HashMap::from([
        (2, vec!['a', 'b', 'c']),
        (3, vec!['d', 'e', 'f']),
        (4, vec!['g', 'h', 'i']),
        (5, vec!['j', 'k', 'l']),
        (6, vec!['m', 'n', 'o']),
        (7, vec!['p', 'q', 'r', 's']),
        (8, vec!['t', 'u', 'v']),
        (9, vec!['w', 'x', 'y', 'z']),
    ]);

    if digits.is_empty() {
        return vec![];
    }

    let mut combinations: Vec<String> = vec!["".to_string()];
    for ch in digits.chars() {
        let num = ch.to_digit(10).unwrap() as i32;
        let letters: Vec<char> = phone_map.get(&num).unwrap().to_vec();

        let mut new_combinations = Vec::new();
        for comb in combinations {
            for letter in &letters {
                new_combinations.push(format!("{}{}", comb, letter));
            }
        }
        combinations = new_combinations;
    }
    combinations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            letter_combinations("23".to_string()),
            vec![
                "ad".to_string(),
                "ae".to_string(),
                "af".to_string(),
                "bd".to_string(),
                "be".to_string(),
                "bf".to_string(),
                "cd".to_string(),
                "ce".to_string(),
                "cf".to_string()
            ]
        );
        // assert_eq!(letter_combinations("".to_string()), vec![]);
        // assert_eq!(
        //     letter_combinations("2".to_string()),
        //     vec!["a".to_string(), "b".to_string(), "c".to_string()]
        // );
    }
}
