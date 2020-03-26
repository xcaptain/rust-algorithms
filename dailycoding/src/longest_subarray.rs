//! This problem was asked by Google.
//! Given an array of elements, return the length of the longest subarray where all its elements are distinct.
//! For example, given the array [5, 1, 3, 5, 2, 3, 4, 1], return 5 as the longest subarray of distinct elements is [5, 2, 3, 4, 1].

pub fn longest_subarray(arr: Vec<i32>) -> usize {
    let l = arr.len();
    let mut res = 0;
    let mut i = 0;
    while i < l {
        for j in i + 1..l {
            let dup_pos = find_dup_pos(&arr, i, j);
            match dup_pos {
                None => {
                    let dis = j - i + 1;
                    res = res.max(dis);
                }
                Some(pos) => {
                    let dis = pos - i + 1;
                    // println!("dbg: {}, {}, {}", j, i, dis);
                    res = res.max(dis);
                    i = pos;
                    break;
                }
            }
        }
        i += 1;
    }
    res
}

fn find_dup_pos(arr: &Vec<i32>, start: usize, end: usize) -> Option<usize> {
    for i in start..end {
        if arr[i] == arr[end] {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_subarray() {
        assert_eq!(5, longest_subarray(vec![5, 1, 3, 5, 2, 3, 4, 1]));
        assert_eq!(5, longest_subarray(vec![5, 2, 3, 4, 1]));
        assert_eq!(1, longest_subarray(vec![5, 5]));
        assert_eq!(0, longest_subarray(vec![]));
    }
}
