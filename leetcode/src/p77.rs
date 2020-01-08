// https://leetcode-cn.com/problems/combinations/

pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];

    backtrack(&mut res, vec![], 1, n, k);
    res
}

fn backtrack(res: &mut Vec<Vec<i32>>, cur: Vec<i32>, start: i32, end: i32, num: i32) {
    if cur.len() == num as usize {
        res.push(cur);
        return;
    }
    for i in start..=end {
        let mut new_cur = cur.clone();
        new_cur.push(i);
        backtrack(res, new_cur, i + 1, end, num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p77() {
        assert_eq!(
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ],
            combine(4, 2)
        );
    }
}
