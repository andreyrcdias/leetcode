fn find_array(pref: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(pref.len());
    let mut prev = 0;
    for num in pref {
        let curr = prev ^ num;
        result.push(curr);
        prev = num;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_array() {
        assert_eq!(find_array(vec![5, 2, 0, 3, 1]), vec![5, 7, 2, 3, 2]);
        assert_eq!(find_array(vec![13]), vec![13]);
    }
}
