use std::collections::HashSet;

fn is_happy(n: i32) -> bool {
    let mut seen = HashSet::new();
    let mut num = n;
    while num != 1 {
        if seen.contains(&num) {
            return false;
        }
        seen.insert(num);

        let mut sum = 0;
        let mut temp = num;
        while temp > 0 {
            let digit = temp % 10;
            sum += digit * digit;
            temp /= 10;
        }
        num = sum;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert_eq!(is_happy(19), true);
        assert_eq!(is_happy(2), false);
    }
}
