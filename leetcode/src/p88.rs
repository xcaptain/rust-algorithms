// https://leetcode-cn.com/problems/merge-sorted-array/

pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    let m = m as usize;
    let n = n as usize;
    let mut nums1_copy = vec![];
    for item in nums1.iter().take(m).skip(0) {
        nums1_copy.push(*item);
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
    for item in nums1_copy.iter().take(m).skip(p1) {
        nums1[p] = *item;
        p += 1;
    }
    for item in nums2.iter().take(n).skip(p2) {
        nums1[p] = *item;
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
