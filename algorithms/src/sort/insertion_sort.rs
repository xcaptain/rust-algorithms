use std::cmp::PartialOrd;

// insertion sort
pub fn insertion_sort<T: PartialOrd + Copy>(mut sorted: Vec<T>) -> Vec<T> {
    let len = sorted.len();
    for i in 0..len {
        let mut pos = i;
        let cursor = sorted[i];
        while pos > 0 && sorted[pos - 1] > cursor {
            sorted[pos] = sorted[pos - 1];
            pos -= 1;
        }
        sorted[pos] = cursor;
    }
    sorted
}

pub fn insertion_sort1(arr: &mut Vec<usize>) {
    let len = arr.len();
    for i in 0..len {
        let mut pos = i;
        let cursor = arr[i];
        while pos > 0 && arr[pos - 1] > cursor {
            arr[pos] = arr[pos - 1];
            pos -= 1;
        }
        arr[pos] = cursor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insertion_sort_normal() {
        assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_insertion_sort_reverse() {
        assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_insertion_sort_random() {
        assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![1, 3, 2, 4]));
    }

    #[test]
    fn test_insertion_sort_string() {
        assert_eq!(
            vec!["1", "2", "3", "4"],
            insertion_sort(vec!["1", "3", "2", "4"])
        );
    }

    #[test]
    fn test_insertion_sort1() {
        let mut arr = vec![4, 3, 2, 1];
        insertion_sort1(&mut arr);
        assert_eq!(vec![1, 2, 3, 4], arr);
    }
}
