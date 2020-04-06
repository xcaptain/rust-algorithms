// https://leetcode-cn.com/problems/longest-consecutive-sequence/

use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut hs = HashSet::new();
    for num in &nums {
        hs.insert(*num);
    }

    let mut res = 0;
    for num in nums {
        if !hs.contains(&(num - 1)) {
            let mut cur_res = 1;
            let mut cur_num = num;
            // make sure num is the smallest element in the consecutive sequence
            while hs.contains(&(cur_num + 1)) {
                cur_num += 1;
                cur_res += 1;
            }
            res = res.max(cur_res);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p128() {
        assert_eq!(4, longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }
}
