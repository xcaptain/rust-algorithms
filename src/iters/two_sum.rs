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
