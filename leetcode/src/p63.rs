/// https://leetcode-cn.com/problems/unique-paths-ii/

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
    if obstacle_grid[0][0] == 0 {
        dp[0][0] = 1;
    } else {
        dp[0][0] = 0;
    }

    for i in 0..m {
        for j in 0..n {
            if i == 0 && j > 0 {
                if obstacle_grid[i][j - 1] == 1 || obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = dp[i][j - 1];
                }
            } else if i > 0 && j == 0 {
                if obstacle_grid[i - 1][j] == 1 || obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            } else if i > 0 && j > 0 {
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }
    }
    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths_with_obstacles() {
        #[derive(Debug, Clone)]
        struct Args {
            obstacle_grid: Vec<Vec<i32>>,
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
                    obstacle_grid: vec![vec![0, 1], vec![0, 0]],
                },
                want: 1,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    obstacle_grid: vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
                },
                want: 2,
            },
            Test {
                name: String::from("case3"),
                args: Args {
                    obstacle_grid: vec![vec![1]],
                },
                want: 0,
            },
            Test {
                name: String::from("case4"),
                args: Args {
                    obstacle_grid: vec![vec![0]],
                },
                want: 1,
            },
        ];

        for test in tests {
            assert_eq!(
                test.want,
                unique_paths_with_obstacles(test.args.obstacle_grid),
                "{} fails",
                test.name
            );
        }
    }
}
