// https://leetcode-cn.com/problems/container-with-most-water/solution/sheng-zui-duo-shui-de-rong-qi-by-leetcode/

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut j = height.len() - 1;
    while i < j {
        let area: i32 = height[i].min(height[j]) * (j - i) as i32;
        ans = ans.max(area);
        // already find largest, to find a larger one, need to decide how to move i and j
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    ans as i32
}

/// brute force solution, very slow
pub fn max_area_v2(height: Vec<i32>) -> i32 {
    let l = height.len();
    let mut ans = 0;
    for i in 0..l - 1 {
        for j in i + 1..l {
            let cur_area: i32 = ((j - i) as i32) * height[i].min(height[j]);
            ans = ans.max(cur_area);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p11() {
        assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));

        assert_eq!(49, max_area_v2(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
