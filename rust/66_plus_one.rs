fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut numbers = digits.clone();
    let size = numbers.len();

    if size == 1 && numbers[0] == 9 {
        return [1,0].to_vec();
    }

    if numbers[size - 1] == 9 {
        numbers = plus_one((&numbers[0..(size-1)]).to_vec());
        numbers.push(0);
    } else {
        numbers[size-1] += 1;
    }

    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus_one() {
        let input = vec![1,2,3];
        let expected = vec![1,2,4];
        let got = plus_one(input);
        assert_eq!(got, expected);
    }

    #[test]
    fn test_plus_two() {
        let input = vec![4,3,2,1];
        let expected = vec![4,3,2,2];
        let got = plus_one(input);
        assert_eq!(got, expected);
    }

    #[test]
    fn test_plus_one_two_digits() {
        let input = vec![9];
        let expected = vec![1,0];
        let got = plus_one(input);
        assert_eq!(got, expected);
    }
}

