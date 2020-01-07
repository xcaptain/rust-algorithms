/// input an array, return the max subarray sum
use std::cmp::max;
use std::isize::MIN as MIN_VALUE;

/// stupid brute force way, O(n^3)
pub fn max_sub_sum_v1(arr: Vec<isize>) -> isize {
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

/// a little improve than v1, O(n^2)
pub fn max_sub_sum_v2(arr: Vec<isize>) -> isize {
    let l = arr.len();
    let mut best = MIN_VALUE;

    for i in 0..l {
        let mut sum = 0;
        for j in arr.iter().take(l).skip(i) {
            sum += j;
            best = max(sum, best);
        }
    }

    best
}

/// linear online-algorithm, O(n)
///      sum                best
/// i=0  a0                 a0
/// i=1  max(a1, prev_sum)  max(sum, prev_best)   largest sequence sum when array length is 1
/// i=2  max(a2, prev_sum)  max(sum, prev_best)   largest sequence sum when array length is 2
pub fn max_sub_sum_v3(arr: Vec<isize>) -> isize {
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

/// max subsequence sum using divide-and-merge way, O(nlog(n))
pub fn max_sub_sum_v4(arr: Vec<isize>) -> isize {
    max_sub_sum_helper(&arr, 0, arr.len() - 1)
}

fn max_sub_sum_helper(arr: &[isize], left: usize, right: usize) -> isize {
    if left == right {
        return arr[left];
    }
    let center = (left + right) / 2;
    let max_left_sum = max_sub_sum_helper(arr, left, center);
    let max_right_sum = max_sub_sum_helper(arr, center + 1, right);

    let mut max_left_border_sum = MIN_VALUE;
    let mut left_border_sum = 0;
    for item in arr.iter().take(center + 1).skip(left).rev() {
        left_border_sum += item;
        if left_border_sum > max_left_border_sum {
            max_left_border_sum = left_border_sum;
        }
    }

    let mut max_right_border_sum = MIN_VALUE;
    let mut right_border_sum = 0;
    // clippy prefer to use iterator
    // for i in center + 1..=right {
    //     right_border_sum += arr[i];
    //     if right_border_sum > max_right_border_sum {
    //         max_right_border_sum = right_border_sum;
    //     }
    // }
    for item in arr.iter().take(right + 1).skip(center + 1) {
        right_border_sum += item;
        if right_border_sum > max_right_border_sum {
            max_right_border_sum = right_border_sum;
        }
    }

    max(
        max(max_left_sum, max_right_sum),
        max_left_border_sum + max_right_border_sum,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_sum() {
        assert_eq!(15, max_sub_sum_v1(vec![1, 2, 3, 4, 5]));
        assert_eq!(15, max_sub_sum_v2(vec![1, 2, 3, 4, 5]));
        assert_eq!(15, max_sub_sum_v3(vec![1, 2, 3, 4, 5]));
        assert_eq!(15, max_sub_sum_v4(vec![1, 2, 3, 4, 5]));

        assert_eq!(-1, max_sub_sum_v1(vec![-1, -2, -3, -4, -5]));
        assert_eq!(-1, max_sub_sum_v2(vec![-1, -2, -3, -4, -5]));
        assert_eq!(-1, max_sub_sum_v3(vec![-1, -2, -3, -4, -5]));
        assert_eq!(-1, max_sub_sum_v4(vec![-1, -2, -3, -4, -5]));

        assert_eq!(5, max_sub_sum_v1(vec![-1, -2, 3, -4, 5]));
        assert_eq!(5, max_sub_sum_v2(vec![-1, -2, 3, -4, 5]));
        assert_eq!(5, max_sub_sum_v3(vec![-1, -2, 3, -4, 5]));
        assert_eq!(
            43,
            max_sub_sum_v3(vec![
                13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7
            ])
        );
        assert_eq!(
            43,
            max_sub_sum_v4(vec![
                13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7
            ])
        );
    }
}
