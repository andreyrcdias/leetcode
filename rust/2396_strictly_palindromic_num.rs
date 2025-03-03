fn is_palindromic(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn to_base(n: i32, base: i32) -> String {
    let mut num = n;
    let mut result = String::new();

    while num > 0 {
        let remainder = num % base;
        result.push_str(&format!("{}", remainder));
        num /= base;
    }
    result.chars().rev().collect()
}

fn is_strictly_palindromic(n: i32) -> bool {
    if n < 3 {
        return false;
    }

    for base in 2..(n - 1) {
        let rep = to_base(n, base);
        if !is_palindromic(&rep) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_strictly_palindromic() {
        assert_eq!(is_strictly_palindromic(9), false);
        assert_eq!(is_strictly_palindromic(4), false);
    }
}
