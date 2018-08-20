use std::cmp::min;

/// 使用尺取法，计算最短连续子数组长度大于某个值
pub fn shortest_seq(arr: Vec<usize>, target: usize) -> (usize, usize, usize) {
    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;
    let len = arr.len();
    let mut ans = len + 1;

    loop {
        while sum < target && end < len {
            sum += arr[end];
            end += 1;
        }
        if sum < target {
            break;
        }
        ans = min(ans, end - start);
        sum -= arr[start];
        start += 1;
    }
    if ans > len {
        return (0, 0, 0);
    }
    return (ans, start - 1, end - 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!((3, 2, 4), shortest_seq(vec![1, 2, 3, 5, 6], 12));
    }

    #[test]
    fn test_2() {
        assert_eq!((2, 3, 4), shortest_seq(vec![1, 2, 3, 5, 6], 11));
    }

    #[test]
    fn test_3() {
        assert_eq!((2, 3, 4), shortest_seq(vec![1, 2, 3, 5, 6], 10));
    }

    #[test]
    fn test_4() {
        assert_eq!((0, 0, 0), shortest_seq(vec![1, 2, 3, 5, 6], 20));
    }
}
