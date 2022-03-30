// https://leetcode-cn.com/problems/subsets-ii/

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];

    let l = nums.len();
    for i in 0..=l {
        let t = subsets_with_length(&nums, i);
        res.extend_from_slice(&t);
    }
    res
}

fn subsets_with_length(nums: &[i32], k: usize) -> Vec<Vec<i32>> {
    let l = nums.len();
    let mut res = vec![];
    backtrack(nums, &mut res, vec![], 0, l, k);
    res
}

fn backtrack(
    nums: &[i32],
    res: &mut Vec<Vec<i32>>,
    cur: Vec<i32>,
    start: usize,
    end: usize,
    k: usize,
) {
    if cur.len() == k {
        let mut cur = cur;
        cur.sort_unstable();
        if !res.contains(&cur) {
            res.push(cur);
        }
        return;
    }
    for i in start..end {
        let mut new_cur = cur.clone();
        new_cur.push(nums[i]);
        backtrack(nums, res, new_cur, i + 1, end, k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p90() {
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![2, 2],
                vec![1, 2, 2],
            ],
            subsets_with_dup(vec![1, 2, 2])
        );
    }
}
