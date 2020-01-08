// https://leetcode-cn.com/problems/rotate-image/

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    for i in 0..rows {
        for j in i..cols {
            let t = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = t;
        }
    }

    for i in 0..rows {
        for j in 0..cols / 2 {
            matrix[i].swap(j, cols - j - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p48() {
        let mut m1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut m1);
        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], m1);
    }
}
