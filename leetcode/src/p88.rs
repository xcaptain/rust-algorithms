// https://leetcode-cn.com/problems/merge-sorted-array/

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let m = m as usize;
    let n = n as usize;
    let mut nums1_copy = vec![];
    for i in 0..(m as usize) {
        nums1_copy.push(nums1[i]);
    }

    let mut p1 = 0;
    let mut p2 = 0;
    let mut p = 0;
    while p1 < m && p2 < n {
        if nums1_copy[p1] < nums2[p2] {
            nums1[p] = nums1_copy[p1];
            p1 += 1;
        } else {
            nums1[p] = nums2[p2];
            p2 += 1;
        }
        p += 1;
    }
    for i in p1..m {
        nums1[p] = nums1_copy[i];
        p += 1;
    }
    for i in p2..n {
        nums1[p] = nums2[i];
        p += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
    }
}
