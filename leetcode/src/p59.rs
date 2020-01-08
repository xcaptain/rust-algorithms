// https://leetcode-cn.com/problems/spiral-matrix-ii/

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; n as usize]; n as usize];
    let mut seen = vec![vec![false; n as usize]; n as usize];

    let mut r = 0_usize;
    let mut c = 0_usize;
    let mut di = 0_usize;

    let dr: Vec<i32> = vec![0, 1, 0, -1];
    let dc: Vec<i32> = vec![1, 0, -1, 0];
    for i in 1..=n * n {
        matrix[r][c] = i;
        seen[r][c] = true;
        // println!("r={}, c={}, di={}", r, c, di);
        let cr = (r as i32) + dr[di];
        let cc = (c as i32) + dc[di];

        if cr >= 0 && cr < n && cc >= 0 && cc < n && !seen[cr as usize][cc as usize] {
            r = cr as usize;
            c = cc as usize;
        } else {
            di = (di + 1) % 4;
            let rr = r as i32 + dr[di];
            r = rr as usize;

            let rc = c as i32 + dc[di];
            c = rc as usize;
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p59() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            generate_matrix(3)
        );
    }
}
