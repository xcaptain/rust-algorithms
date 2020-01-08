// https://leetcode-cn.com/problems/container-with-most-water/solution/sheng-zui-duo-shui-de-rong-qi-by-leetcode/

use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut j = height.len() - 1;
    while i < j {
        let area = min(height[i], height[j]) as usize * (j - i);
        ans = max(ans, area);
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p11() {
        assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
