// https://leetcode-cn.com/problems/xor-queries-of-a-subarray/

pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    for row in queries {
        let a = row[0] as usize;
        let b = row[1] as usize;
        let mut s = 0;
        for i in a..=b {
            s ^= arr[i];
        }
        ans.push(s);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1310() {
        assert_eq!(
            vec![2, 7, 14, 8],
            xor_queries(
                vec![1, 3, 4, 8],
                vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]],
            )
        );
    }
}
