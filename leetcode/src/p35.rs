// https://leetcode-cn.com/problems/search-insert-position/

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (k, v) in nums.iter().enumerate() {
        if v == &target {
            return k as i32;
        }
    }
    // if not in array, find a position to insert
    if target < nums[0] {
        return 0;
    }
    if target > nums[nums.len() - 1] {
        return nums.len() as i32;
    }
    for i in 0..nums.len() - 1 {
        if nums[i] <= target && nums[i + 1] > target {
            return (i + 1) as i32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(2, search_insert(vec![1, 3, 5, 6], 5));
        assert_eq!(1, search_insert(vec![1, 3, 5, 6], 2));
        assert_eq!(4, search_insert(vec![1, 3, 5, 6], 7));
        assert_eq!(0, search_insert(vec![1, 3, 5, 6], 0));
    }
}
