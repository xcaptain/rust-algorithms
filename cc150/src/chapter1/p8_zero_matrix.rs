/// set row and col to zero if an element is zero
pub fn zero_matrix(matrix: &mut Vec<Vec<i32>>) {
    let mut zero_x: Vec<usize> = vec![];
    let mut zero_y: Vec<usize> = vec![];
    let m = matrix.len(); // mxn matrix, row len
    let n = matrix[0].len(); // col len

    for (i, row) in matrix.iter().enumerate().take(m) {
        for (j, val) in row.iter().enumerate().take(n) {
            if val == &0 {
                zero_x.push(i);
                zero_y.push(j);
            }
        }
    }

    for (i, row) in matrix.iter_mut().enumerate().take(m) {
        for (j, val) in row.iter_mut().enumerate().take(n) {
            if zero_x.contains(&i) || zero_y.contains(&j) {
                *val = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_matrix() {
        let mut m1 = vec![vec![1, 2, 3], vec![1, 0, 3], vec![1, 2, 0], vec![1, 2, 3]];
        zero_matrix(&mut m1);
        let m2 = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![1, 0, 0]];
        assert_eq!(m2, m1);

        let mut m3 = vec![
            vec![1, 2, 3, 4, 0],
            vec![6, 0, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 0, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        zero_matrix(&mut m3);
        let m4 = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![11, 0, 13, 14, 0],
            vec![0, 0, 0, 0, 0],
            vec![21, 0, 23, 24, 0],
        ];
        assert_eq!(m4, m3);
    }
}
