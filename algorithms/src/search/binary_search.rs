// 二分查找，输入是有序的，如果查找到了则返回对应的index，否则返回None

// 递归实现
pub fn binary_search_rec(arr: Vec<usize>, target: usize) -> Option<usize> {
    let len = arr.len();
    return binary_search_help(arr, 0, len - 1, target);
}

fn binary_search_help(arr: Vec<usize>, left: usize, right: usize, target: usize) -> Option<usize> {
    if left <= right {
        let mid = (left + right) / 2;
        if arr[mid] < target {
            return binary_search_help(arr, mid + 1, right, target);
        } else if arr[mid] > target {
            return binary_search_help(arr, left, mid - 1, target);
        } else {
            return Some(mid);
        }
    }
    return None;
}

// 迭代实现
pub fn binary_search_iter(arr: Vec<usize>, target: usize) -> Option<usize> {
    let len = arr.len();
    let mut left = 0;
    let mut right = len - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else if arr[mid] > target {
            right = mid - 1;
        } else {
            return Some(mid);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal_rec() {
        assert_eq!(Some(0), binary_search_rec(vec![1, 2, 3], 1));
    }

    #[test]
    fn test_not_found_rec() {
        assert_eq!(None, binary_search_rec(vec![1, 2, 3], 10));
    }

    #[test]
    fn test_greater_rec() {
        assert_eq!(Some(2), binary_search_rec(vec![1, 2, 3], 3));
    }

    // for iter version
    #[test]
    fn test_normal_iter() {
        assert_eq!(Some(0), binary_search_iter(vec![1, 2, 3], 1));
    }

    #[test]
    fn test_not_found_iter() {
        assert_eq!(None, binary_search_iter(vec![1, 2, 3], 10));
    }

    #[test]
    fn test_greater_iter() {
        assert_eq!(Some(2), binary_search_iter(vec![1, 2, 3], 3));
    }
}
