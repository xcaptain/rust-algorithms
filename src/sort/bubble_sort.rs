// 冒泡排序
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
