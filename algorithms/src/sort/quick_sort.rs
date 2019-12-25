// quick sort
pub fn quick_sort(arr: Vec<usize>) -> Vec<usize> {
    let mut sorted = arr;
    let len = sorted.len();
    quick_sort_help(&mut sorted, 0, len - 1);

    sorted
}

fn partition(arr: &mut Vec<usize>, first: usize, last: usize) -> usize {
    let mut wall = first;
    for pos in first..last {
        if arr[pos] < arr[last] {
            arr.swap(pos, wall);
            wall += 1;
        }
    }
    arr.swap(last, wall);
    wall
}

fn quick_sort_help(arr: &mut Vec<usize>, first: usize, last: usize) {
    if first < last {
        let pos = partition(arr, first, last);
        if pos == first {
            quick_sort_help(arr, pos + 1, last);
        } else if pos == last {
            quick_sort_help(arr, first, pos - 1);
        } else {
            quick_sort_help(arr, first, pos - 1);
            quick_sort_help(arr, pos + 1, last);
        }
    }
}

// a link from reddit, it works
// https://www.reddit.com/r/rust/comments/31v6gw/sample_quicksort/cq5du5n/
// pub fn qs<E: Ord>(arr: &mut [E]) {
//     if 1 < arr.len() {
//         let (mut pivot, mut hi) = (0, arr.len()-1);
//         for _ in 0..arr.len()-1 {
//             if arr[pivot] < arr[pivot+1] {
//                 arr.swap(pivot+1, hi);
//                 hi -= 1;
//             } else {
//                 arr.swap(pivot, pivot+1);
//                 pivot += 1;
//             }
//         }
//         qs(&mut arr[..pivot]);
//         qs(&mut arr[pivot+1..]);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(vec![1, 2, 3, 4], quick_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(vec![1, 2, 3, 4], quick_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_random() {
        assert_eq!(vec![1, 2, 3, 4], quick_sort(vec![1, 3, 2, 4]));
    }
}
