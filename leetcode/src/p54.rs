// https://leetcode-cn.com/problems/spiral-matrix/

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let rows = matrix.len();
    if rows == 0 {
        return vec![];
    }
    let cols = matrix[0].len();
    let mut res = vec![];

    let mut seen: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    let dr: Vec<i32> = vec![0, 1, 0, -1];
    let dc: Vec<i32> = vec![1, 0, -1, 0];
    let mut r = 0_usize;
    let mut c = 0_usize;
    let mut di: usize = 0_usize;
    for _i in 0..rows * cols {
        // println!("r={}, c={}", r, c);
        res.push(matrix[r][c]);
        seen[r][c] = true;
        let cr: i32 = (r as i32) + dr[di];
        let cc: i32 = (c as i32) + dc[di];
        // println!("r={}, c={}", cr, cc);

        if cr >= 0
            && cr < (rows as i32)
            && cc >= 0
            && cc < (cols as i32)
            && !seen[cr as usize][cc as usize]
        {
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
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p54() {
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
    }
}
