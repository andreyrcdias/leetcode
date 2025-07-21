fn add_strings(num1: String, num2: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut i = num1.len() as isize - 1;
    let mut j = num2.len() as isize - 1;

    while i >= 0 || j >= 0 || carry > 0 {
        let digit1 = if i >= 0 {
            num1.as_bytes()[i as usize] - b'0'
        } else {
            0
        };
        let digit2 = if j >= 0 {
            num2.as_bytes()[j as usize] - b'0'
        } else {
            0
        };

        let sum = digit1 + digit2 + carry;
        carry = sum / 10;
        result.push((sum % 10 + b'0') as char);

        i -= 1;
        j -= 1;
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_strings() {
        assert_eq!(
            add_strings(String::from("11"), String::from("123")),
            String::from("134")
        );
        assert_eq!(
            add_strings(String::from("456"), String::from("77")),
            String::from("533")
        );
        assert_eq!(
            add_strings(String::from("0"), String::from("0")),
            String::from("0")
        );
        assert_eq!(
            add_strings(
                String::from("3876620623801494171"),
                String::from("6529364523802684779")
            ),
            String::from("10405985147604178950")
        );
    }
}
