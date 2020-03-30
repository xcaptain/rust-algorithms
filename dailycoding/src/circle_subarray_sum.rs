//! This problem was asked by Facebook.
//! Given a circular array, compute its maximum subarray sum in O(n) time.
//! A subarray can be empty, and in this case the sum is 0.
//! For example, given [8, -1, 3, 4], return 15 as we choose the numbers 3, 4, and 8 where the 8 is obtained from wrapping around.
//! Given [-4, 5, 1, 0], return 6 as we choose the numbers 5 and 1.
//!
//! Hint: 2 pointer, if one reaches end then set to 0 and continue

pub fn max_circle_subarray_sum(arr: Vec<i32>) -> i32 {
    let l = arr.len();
    let mut res = 0;

    for i in 0..l {
        let mut t_sum = arr[i];
        let mut j = if i == l - 1 { 0 } else { i + 1 };

        while j != i {
            t_sum += arr[j];
            res = res.max(t_sum);
            if j == l - 1 {
                j = 0;
            } else {
                j += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_circle_subarray_sum() {
        assert_eq!(15, max_circle_subarray_sum(vec![8, -1, 3, 4]));
        assert_eq!(6, max_circle_subarray_sum(vec![-4, 5, 1, 0]));
    }
}
