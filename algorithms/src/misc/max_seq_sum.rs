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
            for k in arr.iter().take(j + 1).skip(i) {
                sum += k;
            }
            best = max(sum, best);
        }
    }
    best
}

// iter 2 times, keep last results
pub fn max_seq_sum_v2(arr: Vec<isize>) -> isize {
    let l = arr.len();
    let mut best = MIN_VALUE;

    for i in 0..l {
        let mut sum = 0;
        for j in arr.iter().take(l).skip(i) {
            // every time j moves, use the previous sum
            sum += j;
            best = max(sum, best);
        }
    }

    best
}

/// iter 1 time
///      sum                best
/// i=0  a0                 a0
/// i=1  max(a1, prev_sum)  max(sum, prev_best)   largest sequence sum when array length is 1
/// i=2  max(a2, prev_sum)  max(sum, prev_best)   largest sequence sum when array length is 2
pub fn max_seq_sum_v3(arr: Vec<isize>) -> isize {
    let mut max_sum = MIN_VALUE;
    let mut this_sum = 0;

    for i in arr {
        // this_sum = max(i, this_sum + i);
        // max_sum = max(max_sum, this_sum);
        this_sum += i;
        if this_sum > max_sum {
            max_sum = this_sum;
        } else if this_sum < 0 {
            this_sum = 0;
        }
    }
    max_sum
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
        assert_eq!(
            43,
            max_seq_sum_v3(vec![
                13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7
            ])
        );
    }
}
