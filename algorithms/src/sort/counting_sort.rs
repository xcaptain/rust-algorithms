// counting sort
// only for array with positive numbers

pub fn counting_sort(arr: Vec<usize>, max_val: usize) -> Vec<usize> {
    let l = arr.len();
    let mut sorted: Vec<usize> = vec![0; l];
    let mut count_arr: Vec<usize> = vec![0; max_val];

    // count_arr的key是待排序数组元素，value是对应元素出现的次数
    for i in 0..l {
        count_arr[arr[i]] += 1;
    }

    // count_arr的key是待排序数组中的元素，value是待排序数组中小于或等于对应元素的值
    for i in 1..max_val {
        count_arr[i] += count_arr[i - 1];
    }

    // 把待排序数组中的每个元素放置到已排序数组中的对应位置上
    for i in 0..l {
        sorted[count_arr[arr[i]] - 1] = arr[i];
        count_arr[arr[i]] -= 1; // 待排序数组中有重复元素时，排完一个，计数数组对应要-1
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(vec![1, 2, 3, 4], counting_sort(vec![1, 2, 3, 4], 5));
        assert_eq!(vec![1, 1, 2, 3, 4], counting_sort(vec![4, 1, 3, 2, 1], 5));
        assert_eq!(vec![1, 2, 3, 4], counting_sort(vec![1, 3, 2, 4], 5));
    }
}
