// https://leetcode-cn.com/problems/combination-sum/

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut candidates = candidates;
    candidates.sort();
    backtrack(candidates, target, &mut res, 0, vec![]);
    res
}

/// backtrack search all possible solutions, tmp keeps the previous sequences
/// res contains all solutions
fn backtrack(
    candidates: Vec<i32>,
    target: i32,
    res: &mut Vec<Vec<i32>>,
    start: usize,
    mut tmp: Vec<i32>,
) {
    if target < 0 {
        return;
    }
    if target == 0 {
        res.push(tmp.clone());
    }
    for i in start..candidates.len() {
        if target < candidates[i] {
            break;
        }
        tmp.push(candidates[i]);
        backtrack(
            candidates.clone(),
            target - candidates[i],
            res,
            i,
            tmp.clone(),
        );
        tmp.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p39() {
        assert_eq!(
            vec![vec![2, 2, 3], vec![7]],
            combination_sum(vec![2, 3, 6, 7], 7)
        );
        assert_eq!(
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            combination_sum(vec![2, 3, 5], 8)
        );
        assert_eq!(
            vec![vec![3, 4, 4], vec![3, 8], vec![4, 7]],
            combination_sum(vec![8, 7, 4, 3], 11)
        );
    }
}
