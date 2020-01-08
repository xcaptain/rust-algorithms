// https://leetcode-cn.com/problems/subsets/

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let l = nums.len();
    let mut res: Vec<Vec<i32>> = vec![];
    for i in 0..=l {
        let t = subsets_with_length(&nums, i);
        res.extend_from_slice(&t);
    }
    res
}

fn subsets_with_length(nums: &Vec<i32>, k: usize) -> Vec<Vec<i32>> {
    let l = nums.len();
    let mut res = vec![];
    backtrack(nums, &mut res, vec![], 0, l, k);
    res
}

fn backtrack(
    nums: &Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    cur: Vec<i32>,
    start: usize,
    end: usize,
    k: usize,
) {
    if cur.len() == k {
        res.push(cur);
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
    fn test_p78() {
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ],
            subsets(vec![1, 2, 3])
        );

        assert_eq!(vec![vec![0_i32; 0],], subsets(vec![]));
    }
}
