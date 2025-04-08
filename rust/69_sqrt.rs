fn my_sqrt(x: i32) -> i32 {
    let n = x as f64;
    let mut guess = n / 10.0;
    let threshold = 10e-5;
    while f64::abs(guess * guess - n) > threshold {
        guess = (guess + (n / guess)) / 2.0;
    }
    guess as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2);
    }
}
