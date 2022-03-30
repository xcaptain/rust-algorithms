// https://leetcode-cn.com/problems/array-partition-i/

pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut res = 0;
    for (k, v) in nums.iter().enumerate() {
        if k % 2 == 0 {
            res += v;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p561() {
        assert_eq!(4, array_pair_sum(vec![1, 4, 3, 2]));
    }
}
