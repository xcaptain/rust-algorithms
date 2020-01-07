// https://leetcode-cn.com/problems/shift-2d-grid/
// have faster solution for this problem

pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut res = grid;
    for _i in 0..k {
        res = shift_once(res);
    }
    res
}

fn shift_once(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len(); // 行数
    let m = grid[0].len(); // 列数
    let mut res = grid.clone();
    for i in 0..n {
        for j in 0..m {
            if i == n - 1 && j == m - 1 {
                res[0][0] = grid[i][j];
            } else if j == m - 1 && i < n - 1 {
                res[i + 1][0] = grid[i][j];
            } else if j < m - 1 {
                res[i][j + 1] = grid[i][j];
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1260() {
        assert_eq!(
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]],
            shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1)
        );

        assert_eq!(
            vec![
                vec![6],
                vec![5],
                vec![1],
                vec![2],
                vec![3],
                vec![4],
                vec![7]
            ],
            shift_grid(
                vec![
                    vec![1],
                    vec![2],
                    vec![3],
                    vec![4],
                    vec![7],
                    vec![6],
                    vec![5]
                ],
                23
            )
        );
    }
}
