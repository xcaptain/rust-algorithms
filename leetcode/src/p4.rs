// https://leetcode-cn.com/problems/median-of-two-sorted-arrays/

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();
    let mut left_cursor = 0;
    let mut right_cursor = 0;
    let mut arr = vec![];
    while left_cursor < m && right_cursor < n {
        if nums1[left_cursor] < nums2[right_cursor] {
            arr.push(nums1[left_cursor]);
            left_cursor += 1;
        } else {
            arr.push(nums2[right_cursor]);
            right_cursor += 1;
        }
    }
    if left_cursor < m {
        for i in left_cursor..m {
            arr.push(nums1[i]);
        }
    }
    if right_cursor < n {
        for i in right_cursor..n {
            arr.push(nums2[i]);
        }
    }
    if (m + n) % 2 == 0 {
        (arr[(m + n) / 2] + arr[(m + n) / 2 - 1]) as f64 / 2_f64
    } else {
        arr[(m + n) / 2] as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p4() {
        assert_eq!(2.0, find_median_sorted_arrays(vec![1, 3], vec![2]));
        assert_eq!(2.5, find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    }
}
