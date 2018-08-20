/// input an array, return the max subarray sum
use std::cmp::max;
use std::isize::MIN as MIN_VALUE;

/// iter 3 times
pub fn max_seq_sum_v1(arr: Vec<isize>) -> isize {
    let l = arr.len();
    let mut best = MIN_VALUE;

    for i in 0..l {
        for j in i..l {
            let mut sum = 0;
            for k in i..j + 1 {
                sum += arr[k];
            }
            best = max(sum, best);
        }
    }
    return best;
}

// iter 2 times, keep last results
pub fn max_seq_sum_v2(arr: Vec<isize>) -> isize {
    let l = arr.len();
    let mut best = MIN_VALUE;

    for i in 0..l {
        let mut sum = 0;
        for j in i..l {
            // every time j moves, use the previous sum
            sum += arr[j];
            best = max(sum, best);
        }
    }

    return best;
}

// iter 1 time
pub fn max_seq_sum_v3(arr: Vec<isize>) -> isize {
    let l = arr.len();
    let mut best = MIN_VALUE;
    let mut sum = 0;

    for i in 0..l {
        sum = max(arr[i], sum + arr[i]);
        best = max(best, sum);
    }
    return best;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        assert_eq!(15, max_seq_sum_v1(vec![1, 2, 3, 4, 5]));
        assert_eq!(15, max_seq_sum_v2(vec![1, 2, 3, 4, 5]));
        assert_eq!(15, max_seq_sum_v3(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_all_neg() {
        assert_eq!(-1, max_seq_sum_v1(vec![-1, -2, -3, -4, -5]));
        assert_eq!(-1, max_seq_sum_v2(vec![-1, -2, -3, -4, -5]));
        assert_eq!(-1, max_seq_sum_v3(vec![-1, -2, -3, -4, -5]));
    }

    #[test]
    fn test_some_neg() {
        assert_eq!(5, max_seq_sum_v1(vec![-1, -2, 3, -4, 5]));
        assert_eq!(5, max_seq_sum_v2(vec![-1, -2, 3, -4, 5]));
        assert_eq!(5, max_seq_sum_v3(vec![-1, -2, 3, -4, 5]));
    }
}
