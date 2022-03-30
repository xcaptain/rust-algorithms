// https://leetcode-cn.com/problems/permutation-sequence/

// TODO: 这个方法很慢提交会超时，得想办法优化到直接定位第k个
pub fn get_permutation(n: i32, k: i32) -> String {
    let mut res: Vec<Vec<i32>> = vec![];
    let arr = (1..=n).collect();
    backtrack(&mut res, vec![], arr);

    // res.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("")
    res.sort_unstable();
    let ans = res[k as usize - 1].clone();
    ans.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn backtrack(res: &mut Vec<Vec<i32>>, prefix: Vec<i32>, cur: Vec<i32>) {
    if cur.is_empty() {
        res.push(prefix);
    } else {
        for i in 0..cur.len() {
            let mut pp = prefix.clone();
            pp.push(cur[i]);

            let mut cc = vec![];
            cc.extend_from_slice(&cur[0..i]);
            cc.extend_from_slice(&cur[i + 1..]);

            backtrack(res, pp, cc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p60() {
        assert_eq!("213", get_permutation(3, 3));
        assert_eq!("2314", get_permutation(4, 9));
    }
}
