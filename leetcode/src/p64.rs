pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // (0, 0) -> (m-1, n-1) 的最短路径是 min{(0, 0) -> (m-2, n-1), (0, 0) -> (m-1, n-2)} + grid[m-1][n-1]
    dp[0][0] = grid[0][0];
    for i in 0..m {
        for j in 0..n {
            if i == 0 && j != 0 {
                dp[i][j] = dp[i][j - 1] + grid[i][j];
            } else if i != 0 && j != 0 {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i][j];
            } else if i != 0 && j == 0 {
                dp[i][j] = dp[i - 1][j] + grid[i][j];
            }
        }
    }
    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        #[derive(Debug, Clone)]
        struct Args {
            grid: Vec<Vec<i32>>,
        }
        #[derive(Debug, Clone)]
        struct Test {
            name: String,
            args: Args,
            want: i32,
        }
        let tests = vec![
            Test {
                name: String::from("case1"),
                args: Args {
                    grid: vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]],
                },
                want: 7,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    grid: vec![vec![1, 2, 3], vec![4, 5, 6]],
                },
                want: 12,
            },
        ];

        for test in tests {
            assert_eq!(
                test.want,
                min_path_sum(test.args.grid),
                "{} fails",
                test.name
            );
        }
    }
}
