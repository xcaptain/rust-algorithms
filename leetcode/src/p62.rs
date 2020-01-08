// https://leetcode-cn.com/problems/unique-paths/

pub fn unique_paths(m: i32, n: i32) -> i32 {
    // let l = (m+n-2) as usize;
    // let mut steps: Vec<String> = vec![];
    // backtrack(m, n, &mut steps, String::new(), l);
    // return steps.len() as i32;

    // using dp
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];
    for i in 0..n as usize {
        dp[0][i] = 1;
    }
    for i in 0..m as usize {
        dp[i][0] = 1;
    }

    for i in 1..m as usize {
        for j in 1..n as usize {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    dp[(m - 1) as usize][(n - 1) as usize]
}

#[allow(dead_code)]
/// will out of time
fn backtrack(m: i32, n: i32, steps: &mut Vec<String>, cur_steps: String, total_steps: usize) {
    // println!("backtrack: {}, {}, {:?}, {}", m, n, steps, cur_steps);
    if cur_steps.len() == total_steps {
        steps.push(cur_steps);
        return;
    }
    if m > 1 {
        let cur_steps = format!("{}d", cur_steps);
        backtrack(m - 1, n, steps, cur_steps, total_steps);
    }
    if n > 1 {
        let cur_steps = format!("{}r", cur_steps);
        backtrack(m, n - 1, steps, cur_steps, total_steps);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p62() {
        assert_eq!(3, unique_paths(3, 2));
        assert_eq!(28, unique_paths(7, 3));
    }
}
