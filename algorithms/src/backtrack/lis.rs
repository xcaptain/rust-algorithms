// longest increasing subsequence
// see also leetcode p300 // https://leetcode-cn.com/problems/longest-increasing-subsequence/

pub fn lis_v1(arr: Vec<i32>) -> usize {
    let mut res = 0;
    for i in 0..arr.len() {
        let l = backtrack_v1(&arr, arr[i], i, arr.len());
        if l > res {
            res = l;
        }
    }

    res
}

fn backtrack_v1(arr: &[i32], prev: i32, start: usize, end: usize) -> usize {
    if start == end {
        return 1;
    }
    if arr[start] <= prev {
        backtrack_v1(arr, prev, start + 1, end)
    } else {
        let skip = backtrack_v1(arr, prev, start + 1, end);
        let take = backtrack_v1(arr, arr[start], start + 1, end) + 1;
        skip.max(take)
    }
}

// v2 of lis
pub fn lis_v2(arr: Vec<i32>) -> usize {
    let mut res = 0;
    for i in 0..arr.len() {
        let l = backtrack_v2(&arr, i, i);
        res = res.max(l);
    }

    res
}

fn backtrack_v2(arr: &[i32], i: usize, j: usize) -> usize {
    let n = arr.len();
    if j >= n {
        return 1;
    }
    if arr[i] >= arr[j] {
        backtrack_v2(arr, i, j + 1)
    } else {
        let skip = backtrack_v2(arr, i, j + 1);
        let take = backtrack_v2(arr, j, j + 1) + 1;
        skip.max(take)
    }
}

/// lis(i) = 1 + {max(lis(j))} where j > i and arr[j] > arr[i]
pub fn lis_v3(arr: Vec<i32>) -> usize {
    let mut res = 0;
    for i in 0..arr.len() {
        let l = backtrack_v3(&arr, i);
        res = res.max(l);
    }

    res
}

fn backtrack_v3(arr: &[i32], i: usize) -> usize {
    let n = arr.len();
    let mut best = 0;
    for j in i + 1..n {
        if arr[j] > arr[i] {
            best = best.max(backtrack_v3(arr, j));
        }
    }
    best + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lis_v1() {
        assert_eq!(3, lis_v1(vec![10, 9, 2, 5, 3, 4])); // 2,3,4
        assert_eq!(0, lis_v1(vec![]));
        assert_eq!(1, lis_v1(vec![1]));
        assert_eq!(1, lis_v1(vec![1, 1])); // 1
        assert_eq!(2, lis_v1(vec![1, 2, 1])); // 1, 2
    }

    #[test]
    fn test_lis_v2() {
        assert_eq!(3, lis_v2(vec![10, 9, 2, 5, 3, 4])); // 2,3,4
        assert_eq!(0, lis_v2(vec![]));
        assert_eq!(1, lis_v2(vec![1]));
        assert_eq!(1, lis_v2(vec![1, 1])); // 1
        assert_eq!(2, lis_v2(vec![1, 2, 1])); // 1, 2
    }

    #[test]
    fn test_lis_v3() {
        assert_eq!(3, lis_v3(vec![10, 9, 2, 5, 3, 4])); // 2,3,4
        assert_eq!(0, lis_v3(vec![]));
        assert_eq!(1, lis_v3(vec![1]));
        assert_eq!(1, lis_v3(vec![1, 1])); // 1
        assert_eq!(2, lis_v3(vec![1, 2, 1])); // 1, 2
    }
}
