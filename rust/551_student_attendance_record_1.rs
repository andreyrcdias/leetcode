fn check_record(s: String) -> bool {
    let mut absents_count = 0;
    let mut lates_count = 0;
    for ch in s.chars() {
        if ch == 'A' {
            absents_count += 1;
        }
        if ch == 'L' {
            lates_count += 1;
            if lates_count >= 3 {
                return false;
            }
        } else {
            lates_count = 0;
        }
    }
    absents_count < 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_record() {
        assert_eq!(check_record("PPALLP".to_string()), true);
        assert_eq!(check_record("PPALLL".to_string()), false);
    }
}
