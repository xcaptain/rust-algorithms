// insertion sort
pub fn insertion_sort(arr: Vec<usize>) -> Vec<usize> {
    let mut sorted = arr.clone();
    let len = sorted.len();
    for i in 0..len {
        let cursor = sorted[i];
        let mut pos = i;
        while pos > 0 && sorted[pos - 1] > cursor {
            sorted[pos] = sorted[pos - 1];
            pos = pos - 1;
        }
        sorted[pos] = cursor
    }
    return sorted;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_random() {
        assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![1, 3, 2, 4]));
    }
}
