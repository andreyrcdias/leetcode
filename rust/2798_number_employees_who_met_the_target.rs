fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    let mut count = 0;
    for hour in hours {
        if hour >= target {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_of_employees_who_met_target() {
        assert_eq!(
            number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2),
            3
        );
        assert_eq!(
            number_of_employees_who_met_target(vec![5, 1, 4, 2, 2], 6),
            0
        );
    }
}
