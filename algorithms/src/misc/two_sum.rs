/// 这个问题来自leetcoder https://leetcode.com/problems/two-sum/solution/
/// 最好的解法是使用2次哈希表，使得算法的时间复杂度为O(n)

pub fn two_sum(arr: Vec<usize>, target: usize) -> (usize, usize) {
    for (i, item) in arr.iter().enumerate() {
        let complement = target - item;
        if arr.contains(&complement) {
            let j = arr.binary_search(&complement).unwrap();
            if j != i {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!((0, 2), two_sum(vec![1, 2, 3], 4));
    }

    #[test]
    fn test_duplicate_elem() {
        assert_eq!((0, 2), two_sum(vec![2, 1, 2, 3], 4));
    }

    #[test]
    fn test_more_duplicate_elem() {
        // 这里返回的不是0, 1是因为rust的数组按值查找用的是binary_search
        // 所以找到的index不一定是第一次出现的位置
        assert_eq!((0, 2), two_sum(vec![2, 2, 2, 3], 4));
    }
}
