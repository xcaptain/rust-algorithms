// https://leetcode-cn.com/problems/rotate-array/

// TODO: better algorithms exist
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let l = nums.len();
    for _i in 0..k {
        let last_elem = nums[l - 1];
        for j in (1..l).rev() {
            nums[j] = nums[j - 1];
        }
        nums[0] = last_elem;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut v1 = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut v1, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], v1);

        let mut v2 = vec![-1, -100, 3, 99];
        rotate(&mut v2, 2);
        assert_eq!(vec![3, 99, -1, -100], v2);
    }
}
