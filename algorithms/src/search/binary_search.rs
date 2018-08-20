// binary search using recursion
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

// binary search using iteration
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
    fn test_binary_search_rec_yes() {
        assert_eq!(Some(0), binary_search_rec(vec![1, 2, 3], 1));
    }

    #[test]
    fn test_binary_search_rec_no() {
        assert_eq!(None, binary_search_rec(vec![1, 2, 3], 10));
    }

    #[test]
    fn test_binary_search_rec_last() {
        assert_eq!(Some(2), binary_search_rec(vec![1, 2, 3], 3));
    }

    // for iter version
    #[test]
    fn test_binary_search_iter_yes() {
        assert_eq!(Some(0), binary_search_iter(vec![1, 2, 3], 1));
    }

    #[test]
    fn test_binary_search_iter_no() {
        assert_eq!(None, binary_search_iter(vec![1, 2, 3], 10));
    }

    #[test]
    fn test_binary_search_iter_last() {
        assert_eq!(Some(2), binary_search_iter(vec![1, 2, 3], 3));
    }
}
