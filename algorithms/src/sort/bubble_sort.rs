// bubble sort
pub fn bubble_sort(arr: Vec<usize>) -> Vec<usize> {
    let mut sorted = arr.clone();
    let len = sorted.len();

    for i in 0..len - 1 {
        for j in i..len {
            if sorted[i] >= sorted[j] {
                let t = sorted[i];
                sorted[i] = sorted[j];
                sorted[j] = t;
            }
        }
    }
    return sorted;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_random() {
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![1, 3, 2, 4]));
    }
}
