// https://leetcode-cn.com/problems/remove-element/
// remove_mut method is still in nightly till 2019-07-03

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    while let Some(pos) = nums.iter().position(|x| *x == val) {
        nums.remove(pos);
    }
    return nums.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut a1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let l = remove_element(&mut a1, 2);
        assert_eq!(l, 5);
        assert_eq!(vec![0, 1, 3, 0, 4], a1);
    }
}
