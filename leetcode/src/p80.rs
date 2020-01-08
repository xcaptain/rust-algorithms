// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array-ii/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let [mut j, mut count] = [1, 1];
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            count += 1;
        } else {
            count = 1;
        }
        if count <= 2 {
            nums[j] = nums[i];
            j += 1;
        }
    }
    j as i32
}

pub fn remove_duplicates_skip(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }
    let mut j = 2;
    for i in 2..nums.len() {
        if nums[i] != nums[j - 2] {
            // 最多连续两个相等
            nums[j] = nums[i];
            j += 1;
        }
    }
    j as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p80() {
        let mut arr1 = vec![1, 1, 1, 2, 2, 3];
        let len1 = remove_duplicates(&mut arr1);
        assert_eq!(5, len1);
        assert_eq!(vec![1, 1, 2, 2, 3], &arr1[0..len1 as usize]);

        let mut arr2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let len2 = remove_duplicates(&mut arr2);
        assert_eq!(7, len2);
        assert_eq!(vec![0, 0, 1, 1, 2, 3, 3], &arr2[0..len2 as usize]);
    }

    #[test]
    fn test_p80_skip() {
        let mut arr1 = vec![1, 1, 1, 2, 2, 3];
        let len1 = remove_duplicates_skip(&mut arr1);
        assert_eq!(5, len1);
        assert_eq!(vec![1, 1, 2, 2, 3], &arr1[0..len1 as usize]);

        let mut arr2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let len2 = remove_duplicates_skip(&mut arr2);
        assert_eq!(7, len2);
        assert_eq!(vec![0, 0, 1, 1, 2, 3, 3], &arr2[0..len2 as usize]);
    }
}
