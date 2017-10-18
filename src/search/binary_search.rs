// 二分查找，输入是有序的，如果查找到了则返回对应的index，否则返回None

// 递归实现
pub fn binary_search_rec(arr: Vec<usize>, target: usize) -> Option<usize> {
    let len = arr.len();
    return binary_search_help(arr, 0, len-1, target);
}

fn binary_search_help(arr: Vec<usize>, left: usize, right: usize, target: usize) -> Option<usize> {
    if left <= right {
        let mid = (left + right) / 2;
        if arr[mid] < target {
            return binary_search_help(arr, mid+1, right, target);
        } else if arr[mid] > target {
            return binary_search_help(arr, left, mid-1, target);
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
            left = mid+1;
        } else if arr[mid] > target {
            right = mid-1;
        } else {
            return Some(mid);
        }
    }
    return None;
}
