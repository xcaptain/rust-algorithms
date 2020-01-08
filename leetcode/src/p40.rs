// https://leetcode-cn.com/problems/combination-sum-ii/

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut res = vec![];
    backtrack(candidates, target, &mut res, 0, vec![]);
    res
}

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
        res.push(tmp);
        return;
    }
    for i in start..candidates.len() {
        if target < candidates[i] {
            break;
        }
        if i > start && candidates[i - 1] == candidates[i] {
            continue;
        }
        tmp.push(candidates[i]);
        backtrack(
            candidates.clone(),
            target - candidates[i],
            res,
            i + 1,
            tmp.clone(),
        );
        tmp.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p40() {
        assert_eq!(
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
            combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
        );

        assert_eq!(
            vec![vec![1, 2, 2], vec![5]],
            combination_sum2(vec![2, 5, 2, 1, 2], 5)
        );

        assert_eq!(
            vec![vec![1, 1, 1, 5], vec![1, 1, 3, 3], vec![3, 5]],
            combination_sum2(vec![3, 1, 3, 5, 1, 1], 8)
        );
    }
}
