// priority queue
use algorithms::sort::heap_sort::max_heapify;

pub fn max_heap_insert(queue: &mut Vec<usize>, num: usize) {
    let old_size = queue.len();
    queue.push(0); // increase queue size
    heap_increase_key(queue, old_size, num);
}

pub fn max_heap_extract(queue: &mut Vec<usize>) -> usize {
    let largest = queue.swap_remove(0); // 返回第0个元素，删除第0个元素，用最末尾元素替代
    let size = queue.len();
    max_heapify(queue, size, 0); // 重新建堆
    largest
}

// 把第i个位置的值设置为key，只能增加不能减少
fn heap_increase_key(queue: &mut Vec<usize>, pos: usize, key: usize) {
    if queue[pos] > key {
        panic!("只能增加不能减少");
    }
    queue[pos] = key;
    let mut i = pos;
    while i > 0 && queue[parent(i)] < queue[i] {
        queue.swap(parent(i), i);
        i = parent(i);
    }
}

fn parent(i: usize) -> usize {
    (i - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let mut queue = vec![];
        max_heap_insert(&mut queue, 4);
        max_heap_insert(&mut queue, 1);
        max_heap_insert(&mut queue, 3);
        max_heap_insert(&mut queue, 2);

        // println!("arr={:?}", queue);
        assert_eq!(4, max_heap_extract(&mut queue));
        assert_eq!(3, max_heap_extract(&mut queue));

        // println!("arr={:?}", queue);
        max_heap_insert(&mut queue, 5);
        max_heap_insert(&mut queue, 1);

        // println!("arr={:?}", queue);
        assert_eq!(5, max_heap_extract(&mut queue));
        // println!("arr={:?}", queue);
    }
}
