// https://leetcode-cn.com/problems/max-increase-to-keep-city-skyline/

pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut horizon_line = vec![0; m];
    let mut vertical_line = vec![0; n];

    for i in 0..m {
        let mut max = 0;
        for j in 0..n {
            if grid[i][j] > max {
                max = grid[i][j];
            }
        }
        horizon_line[i] = max;
    }

    for j in 0..n {
        let mut max = 0;
        for i in 0..m {
            if grid[i][j] > max {
                max = grid[i][j];
            }
        }
        vertical_line[j] = max;
    }

    let mut inc = 0;
    for i in 0..m {
        for j in 0..n {
            inc += horizon_line[i].min(vertical_line[j]) - grid[i][j];
        }
    }

    inc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_increase_keeping_skyline() {
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

        let tests = vec![Test {
            name: String::from("case1"),
            args: Args {
                grid: vec![
                    vec![3, 0, 8, 4],
                    vec![2, 4, 5, 7],
                    vec![9, 2, 6, 3],
                    vec![0, 3, 1, 0],
                ],
            },
            want: 35,
        }];

        for test in tests {
            assert_eq!(
                test.want,
                max_increase_keeping_skyline(test.args.grid),
                "{} fails",
                test.name
            );
        }
    }
}
