// https://leetcode-cn.com/problems/ones-and-zeroes/solution/yi-he-ling-by-leetcode/

use std::cmp::max;

pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let row: Vec<i32> = vec![0; n as usize + 1];
    let mut dp: Vec<Vec<i32>> = vec![row; m as usize + 1];

    for s in strs {
        let counts = zeroandones(s);
        for zeros in (counts[0]..=m).rev() {
            for ones in (counts[1]..=n).rev() {
                dp[zeros as usize][ones as usize] = max(
                    1 + dp[(zeros - counts[0]) as usize][(ones - counts[1]) as usize],
                    dp[zeros as usize][ones as usize],
                );
            }
        }
    }
    dp[m as usize][n as usize]
}

fn zeroandones(s: String) -> [i32; 2] {
    let mut zeros: i32 = 0;
    let mut ones: i32 = 0;
    for c in s.chars().into_iter() {
        if c == '0' {
            zeros += 1;
        } else {
            ones += 1;
        }
    }
    [zeros, ones]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p474() {
        assert_eq!(
            4,
            find_max_form(
                vec![
                    String::from("10"),
                    String::from("0001"),
                    String::from("111001"),
                    String::from("1"),
                    String::from("0")
                ],
                5,
                3
            )
        );
    }
}
