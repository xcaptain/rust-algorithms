//! This problem was asked by Microsoft.
//! Given a number in the form of a list of digits, return all possible permutations.
//! For example, given [1,2,3], return [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]].
//!
//! A simple backtrack algorithms
//! permute(A[0..n]) = A[0] + permute(A[1..n]) ...

pub fn permute(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    backtrack(&arr, vec![], &mut res);
    res
}

fn backtrack(arr: &[i32], cur: Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if arr.is_empty() {
        // let new_cur = vec![arr[0]];
        res.push(cur);
        return;
    }
    for i in 0..arr.len() {
        let mut new_arr = arr.to_owned();
        new_arr.remove(i);

        let mut new_cur = cur.clone();
        new_cur.push(arr[i]);
        backtrack(&new_arr, new_cur, res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
            permute(vec![1, 2, 3])
        );
    }
}
