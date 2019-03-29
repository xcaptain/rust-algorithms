/// search target in array, if found then return some index
/// else return None
pub fn binary_search1(arr: Vec<isize>, target: isize) -> Option<usize> {
    let len = arr.len();
    binary_search1_inner(arr, 0, len - 1, target)
}

fn binary_search1_inner(
    arr: Vec<isize>,
    left: usize,
    right: usize,
    target: isize,
) -> Option<usize> {
    if left <= right {
        let mid = (left + right) / 2;
        if arr[mid] < target {
            return binary_search1_inner(arr, mid + 1, right, target);
        } else if arr[mid] > target {
            return binary_search1_inner(arr, left, mid - 1, target);
        } else {
            return Some(mid);
        }
    }
    None
}

// binary search using iteration
pub fn binary_search2(arr: Vec<isize>, target: isize) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

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
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_search1_yes() {
        assert_eq!(Some(0), binary_search1(vec![1, 2, 3], 1));
    }

    #[test]
    fn test_binary_search1_no() {
        assert_eq!(None, binary_search1(vec![1, 2, 3], 10));
    }

    #[test]
    fn test_binary_search1_last() {
        assert_eq!(Some(2), binary_search1(vec![1, 2, 3], 3));
    }

    // for iter version
    #[test]
    fn test_binary_search2_yes() {
        assert_eq!(Some(0), binary_search2(vec![1, 2, 3], 1));
    }

    #[test]
    fn test_binary_search2_no() {
        assert_eq!(None, binary_search2(vec![1, 2, 3], 10));
    }

    #[test]
    fn test_binary_search2_last() {
        assert_eq!(Some(2), binary_search2(vec![1, 2, 3], 3));
    }
}
