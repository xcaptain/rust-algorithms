// 选择排序
// 先设置数组第一个元素为最小的，数组左半部分是有序的，右半部分是无序的
// 从右半部分数组中找出最小的元素，左半部分最右边元素交换位置
pub fn selection_sort(arr: Vec<usize>) -> Vec<usize> {
    let mut sorted = arr.clone();
    let len = sorted.len();

    for i in 0..len {
        let mut minium = i;
        for j in i+1 .. len {
            if sorted[j] < sorted[minium] {
                minium = j;
            }
        }
        let t = sorted[minium];
        sorted[minium] = sorted[i];
        sorted[i] = t;
    }
    return sorted;
}
