// https://leetcode-cn.com/problems/permutations/

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    backtrack(vec![], nums, &mut res);
    res
}

fn backtrack(prefix: Vec<i32>, cur: Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if cur.len() == 0 {
        res.push(prefix);
    } else {
        for i in 0..cur.len() {
            let mut p1 = prefix.to_owned();
            p1.push(cur[i]);

            let mut p2 = cur[0..i].to_owned();
            p2.extend_from_slice(&cur[i + 1..]);

            backtrack(p1, p2, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p46() {
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
            permute(vec![1, 2, 3])
        );
    }
}
