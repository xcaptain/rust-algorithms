// https://leetcode-cn.com/problems/triangle/

use std::cmp::min;

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let n = triangle.len();
    let mut triangle = triangle;
    for i in (0..n - 1).rev() {
        for j in 0..triangle[i].len() {
            triangle[i][j] += min(triangle[i + 1][j], triangle[i + 1][j + 1]);
        }
    }
    triangle[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p120() {
        assert_eq!(
            11,
            minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        );

        assert_eq!(
            -1,
            minimum_total(vec![vec![-1], vec![2, 3], vec![1, -1, -3]])
        );
    }
}
