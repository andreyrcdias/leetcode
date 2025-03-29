fn can_alice_win(nums: Vec<i32>) -> bool {
    let mut sd_sum = 0;
    let mut dd_sum = 0;
    for num in nums {
        if num / 10 == 0 {
            sd_sum += num;
        } else {
            dd_sum += num;
        }
    }
    sd_sum != dd_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_alice_win() {
        assert_eq!(can_alice_win(vec![1, 2, 3, 4, 10]), false);
        assert_eq!(can_alice_win(vec![1, 2, 3, 4, 5, 14]), true);
        assert_eq!(can_alice_win(vec![5, 5, 5, 25]), true);
    }
}
