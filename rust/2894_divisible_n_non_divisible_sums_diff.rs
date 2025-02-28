fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;
    for i in 1..=n {
        if i % m == 0 {
            num2 += i;
        } else {
            num1 += i;
        }
    }
    num1 - num2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_difference_of_sums() {
        assert_eq!(difference_of_sums(10, 3), 19);
        assert_eq!(difference_of_sums(5, 6), 15);
        assert_eq!(difference_of_sums(5, 1), -15);
    }
}
