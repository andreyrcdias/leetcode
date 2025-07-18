fn is_self_dividing(num: i32) -> bool {
    let mut n = num;

    while n > 0 {
        let digit = n % 10;
        if digit == 0 || num % digit != 0 {
            return false;
        }
        n /= 10;
    }

    true
}

fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut self_div_nums = Vec::new();

    for num in left..=right {
        if is_self_dividing(num) {
            self_div_nums.push(num);
        }
    }

    self_div_nums
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_self_dividing_numbers() {
        assert_eq!(
            self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
        assert_eq!(self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
    }
}
