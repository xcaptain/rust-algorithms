/// get all the possible subsets such that they adds up to target
/// every element in the array is positive and the target is positive
pub fn subset_sum_v1(arr: Vec<usize>, target: usize) -> Vec<Vec<usize>> {
    let mut res = vec![];
    backtrack_v1(&arr, target, 0, arr.len(), vec![], &mut res);
    res
}

fn backtrack_v1(
    arr: &Vec<usize>,
    target: usize,
    start: usize,
    end: usize,
    cur: Vec<usize>,
    res: &mut Vec<Vec<usize>>,
) {
    let cur_sum = cur.iter().sum::<usize>();
    // println!("{:?}, {}", cur, cur_sum);
    if cur_sum == target {
        res.push(cur);
        return;
    }
    for i in start..end {
        let new_sum = cur_sum + arr[i];
        if new_sum > target {
            continue;
        }
        let mut new_cur = cur.clone();
        new_cur.push(arr[i]);
        backtrack_v1(arr, target, i + 1, end, new_cur.clone(), res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_sum_v1() {
        assert_eq!(
            vec![vec![6, 9], vec![8, 7], vec![7, 5, 3], vec![5, 10]],
            subset_sum_v1(vec![6, 8, 7, 5, 3, 10, 9], 15)
        );
    }
}
