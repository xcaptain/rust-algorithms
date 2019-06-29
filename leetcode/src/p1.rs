/// 这个问题来自leetcoder https://leetcode.com/problems/two-sum/solution/
/// 最好的解法是使用2次哈希表，使得算法的时间复杂度为O(n)

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, item) in nums.iter().enumerate() {
        let complement = target - item;
        if nums.contains(&complement) {
            if let Some(j) = nums.iter().position(|&p| p == complement) {
                if j != i {
                    return vec![i as i32, j as i32];
                }
            }
        }
    }
    vec![0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(vec![0, 2], two_sum(vec![1, 2, 3], 4));
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![2, 4], two_sum(vec![-1, -2, -3, -4, -5], -8));
    }
}
