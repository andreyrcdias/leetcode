fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    if mat.is_empty() || r * c != (mat.len() as i32) * (mat[0].len() as i32) {
        return mat;
    }

    let mut reshaped_mat = vec![vec![0; c as usize]; r as usize];
    let mut k = 0;
    for row in mat {
        for num in row {
            reshaped_mat[(k / c) as usize][(k % c) as usize] = num;
            k += 1;
        }
    }
    reshaped_mat
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_reshape() {
        assert_eq!(
            matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
        assert_eq!(
            matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
            vec![vec![1, 2], vec![3, 4]]
        );
    }
}
