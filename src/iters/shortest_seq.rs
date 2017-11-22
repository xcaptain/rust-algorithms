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
    return (ans, start-1, end-1);
}
