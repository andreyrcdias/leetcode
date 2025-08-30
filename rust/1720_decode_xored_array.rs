fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut decodeds = vec![first];
    for &encoded_element in &encoded {
        let next_number = *decodeds.last().unwrap() ^ encoded_element;
        decodeds.push(next_number);
    }
    decodeds
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
        assert_eq!(decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
    }
}
