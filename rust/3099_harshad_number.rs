fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    let (mut sum, mut y) = (0, x);

    while y > 0 {
        sum += y % 10;
        y /= 10;
    }

    if x % sum == 0 {
        sum
    } else {
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_the_digits_of_harshad_number() {
        assert_eq!(sum_of_the_digits_of_harshad_number(18), 9);
        assert_eq!(sum_of_the_digits_of_harshad_number(23), -1);
    }
}
