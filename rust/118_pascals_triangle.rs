fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut triangle = Vec::new();
    triangle.push(vec![1]);

    for _ in 0..num_rows - 1 {
        let last_row = &triangle[triangle.len() - 1];
        let mut temp = vec![1];

        for i in 0..last_row.len() - 1 {
            temp.push(last_row[i] + last_row[i + 1]);
        }
        temp.push(1);
        triangle.push(temp);
    }
    triangle
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_pascal_triangle() {
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
        assert_eq!(generate(1), vec![vec![1]]);
    }
}
