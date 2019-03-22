use std::cmp::PartialOrd;

// bubble sort
pub fn bubble_sort<T: PartialOrd>(mut sorted: Vec<T>) -> Vec<T> {
    let len = sorted.len();
    for i in 0..len {
        for j in i..len {
            if sorted[i].gt(&sorted[j]) {
                sorted.swap(i, j);
            }
        }
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort_normal() {
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_bubble_sort_reverse() {
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_bubble_sort_random() {
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![1, 3, 2, 4]));
    }

    #[test]
    fn test_bubble_sort_string() {
        assert_eq!(
            vec!["1", "2", "3", "4"],
            bubble_sort(vec!["1", "3", "2", "4"])
        );
    }
}
