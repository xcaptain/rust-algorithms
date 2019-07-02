// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/

// the simplest way is to use Vec standard library api `dedup`
// but there should be more complex ways to solve this problem

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    return nums.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut a = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let l = remove_duplicates(&mut a);
        assert_eq!(5, l);
        assert_eq!(vec![0, 1, 2, 3, 4], a);
    }
}
