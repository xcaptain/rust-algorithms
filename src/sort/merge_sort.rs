// 归并排序
// 先把数组打散，然后再按顺序合并数组
pub fn merge_sort_v1(arr: Vec<usize>) -> Vec<usize> {
    let len = arr.len();
    if len <= 1 {
        return arr;
    }
    let mid = len/2;
    let left = merge_sort_v1(arr.get(..mid).unwrap().to_vec());
    let right = merge_sort_v1(arr.get(mid..).unwrap().to_vec());

    return merge_v1(left, right);
}

pub fn merge_sort_v2(arr: Vec<usize>) -> Vec<usize> {
    let len = arr.len();
    if len <= 1 {
        return arr;
    }
    let mid = len/2;
    let left = merge_sort_v2(arr.get(..mid).unwrap().to_vec());
    let right = merge_sort_v2(arr.get(mid..).unwrap().to_vec());

    return merge_v2(left, right);
}

// 移动游标位置而不修改数组本身
// TODO: 想出一个不断截取自身从而实现合并的算法
// RUST的所有权机制导致改变自身太困难了
fn merge_v1(left: Vec<usize>, right: Vec<usize>) -> Vec<usize> {
    let mut res = vec![];
    let left_len = left.len();
    let right_len = right.len();
    let mut left_cursor = 0;
    let mut right_cursor = 0;

    while left_cursor < left_len && right_cursor < right_len {
        if left[left_cursor] < right[right_cursor] {
            res.push(left[left_cursor]);
            left_cursor += 1;
        } else {
            res.push(right[right_cursor]);
            right_cursor += 1;
        }
    }
    for i in left_cursor..left_len {
        res.push(left[i]);
    }
    for i in right_cursor..right_len {
        res.push(right[i])
    }
    return res;
}

fn merge_v2(mut left: Vec<usize>, mut right: Vec<usize>) -> Vec<usize> {
    let mut res = vec![];
    while !left.is_empty() && !right.is_empty() {
        if left[0] <= right[0] {
            res.push(left[0]);
            left.remove(0);
        } else {
            res.push(right[0]);
            right.remove(0);
        }
    }
    if !left.is_empty() {
        res.extend(&left);
    }
    if !right.is_empty() {
        res.extend(&right);
    }
    return res;
}
