// insertion sort
pub fn insertion_sort(arr: Vec<usize>) -> Vec<usize> {
    let mut sorted = arr.clone();
    let len = sorted.len();
    for i in 0..len {
        let cursor = sorted[i];
        let mut pos = i;
        while pos > 0 && sorted[pos-1] > cursor {
            sorted[pos] = sorted[pos-1];
            pos = pos - 1;
        }
        sorted[pos] = cursor
    }
    return sorted;
}
