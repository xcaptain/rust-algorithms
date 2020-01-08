// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let l = nums.len();
    if l <= 1 {
        return l as i32;
    }
    let mut j = 1; // 慢指针，顺序更新自身
    for i in 1..l {
        if nums[i] != nums[j - 1] {
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
    fn test_p26() {
        let mut a = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let l = remove_duplicates(&mut a);
        assert_eq!(5, l);
        assert_eq!(vec![0, 1, 2, 3, 4], &a[0..l as usize]);
    }
}
