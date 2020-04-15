pub fn max_qps(arr: Vec<f64>) -> usize {
    let l = arr.len();
    let mut res = 0;

    for i in 0..l {
        let start_time = arr[i];
        let end_time = start_time + 1.0;

        let mut j = i;
        while j < l && arr[j] >= start_time && arr[j] < end_time {
            j += 1;
        }
        res = res.max(j - i);
    }
    res
}

pub fn max_qps_v2(arr: Vec<f64>) -> usize {
    // 尺取法，2指针移动
    let mut i = 0;
    let mut j = 0;
    let l = arr.len();
    let mut res = 0;
    while i < l {
        let start_time = arr[i];
        let end_time = start_time + 1.0;

        while j < l && arr[j] >= start_time && arr[j] < end_time {
            j += 1;
        }
        let dist = j - i;
        res = res.max(dist);
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_qps() {
        assert_eq!(3, max_qps(vec![1.1, 1.2, 1.3, 2.1, 2.2]));
        assert_eq!(4, max_qps(vec![1.1, 1.2, 1.3, 2.1, 2.2, 2.2]));

        assert_eq!(3, max_qps_v2(vec![1.1, 1.2, 1.3, 2.1, 2.2]));
        assert_eq!(4, max_qps_v2(vec![1.1, 1.2, 1.3, 2.1, 2.2, 2.2]));
        assert_eq!(5, max_qps_v2(vec![1.1, 1.2, 1.3, 2.1, 2.1, 2.1, 2.2, 2.3]));
    }
}
