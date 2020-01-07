// https://leetcode-cn.com/problems/set-matrix-zeroes/

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut cords: Vec<[i32; 2]> = vec![]; // 0点坐标列表

    for (i, row) in matrix.iter().enumerate().take(m).skip(0) {
        for (j, val) in row.iter().enumerate().take(n).skip(0) {
            if val == &0 {
                cords.push([i as i32, j as i32]);
            }
        }
    }
    for cord in cords {
        let a = cord[0];
        let b = cord[1];

        // for i in 0..m {
        //     matrix[i][b as usize] = 0;
        // }
        for row in matrix.iter_mut().take(m) {
            row[b as usize] = 0;
        }
        for j in 0..n {
            matrix[a as usize][j] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p73() {
        let mut v = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut v);
        assert_eq!(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]], v);
    }
}
