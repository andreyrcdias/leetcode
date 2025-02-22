fn flip(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut flipped: Vec<Vec<i32>> = Vec::new();
    for mut row in image {
        row.reverse();
        flipped.push(row);
    }
    flipped
}

fn invert(flipped: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut inverted: Vec<Vec<i32>> = Vec::new();
    for row in flipped {
        let inverted_row: Vec<i32> = row
            .into_iter()
            .map(|x| if x == 0 { 1 } else { 0 })
            .collect();
        inverted.push(inverted_row);
    }
    inverted
}

fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let flipped = flip(image.clone());
    invert(flipped)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flip_and_invert_image() {
        assert_eq!(
            flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
        );

        assert_eq!(
            flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ]),
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ]
        );
    }
}
