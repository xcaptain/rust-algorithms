/// heap sort
/// 1. build max heap
/// 2. swap the last with the last element
/// 3. decrease heap size and rebuild max heap
pub fn heap_sort(arr: &mut Vec<usize>) {
    build_max_heap(arr);
    let mut i: usize = arr.len() - 1;
    while i > 0 {
        arr.swap(0, i);
        max_heapify(arr, i, 0);
        i -= 1;
    }
}

fn build_max_heap(arr: &mut Vec<usize>) {
    let n = arr.len();
    let mut i: usize = n / 2 - 1;
    while i > 0 {
        max_heapify(arr, n, i);
        i -= 1;
    }
    max_heapify(arr, n, i);
}

fn max_heapify(arr: &mut Vec<usize>, size: usize, i: usize) {
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut largest = i;
    if left < size && arr[left] > arr[largest] {
        largest = left;
    }
    if right < size && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != i {
        arr.swap(largest, i);
        max_heapify(arr, size, largest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let mut arr = vec![2, 4, 3, 1];
        heap_sort(&mut arr);
        assert_eq!(1, arr[0]);
    }
}
