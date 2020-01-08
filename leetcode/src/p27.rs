// https://leetcode-cn.com/problems/remove-element/
// remove_mut method is still in nightly till 2019-07-03

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    while let Some(pos) = nums.iter().position(|x| *x == val) {
        nums.remove(pos);
    }
    nums.len() as i32
}

pub fn remove_element_skip(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut j = 0;
    let l = nums.len();
    for i in 0..l {
        // 遍历数组，找出跟val不同的元素，顺序更新自身
        if nums[i] != val {
            nums[j] = nums[i];
            j += 1;
        }
        // println!("{:?}", nums);
    }

    j as i32
}

pub fn remove_element_swap(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut start = 0;
    let mut end = nums.len();

    while start < end {
        if nums[start] == val {
            nums[start] = nums[end - 1];
            end -= 1;
        } else {
            start += 1;
        }
    }
    start as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p27() {
        let mut a1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let l = remove_element(&mut a1, 2);
        assert_eq!(5, l);
        assert_eq!(vec![0, 1, 3, 0, 4], a1);
    }

    #[test]
    fn test_p27_skip() {
        let mut a1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let l = remove_element_skip(&mut a1, 2);
        assert_eq!(5, l);
        assert_eq!(vec![0, 1, 3, 0, 4], &a1[0..l as usize]);

        let mut a2 = vec![3, 2, 2, 3];
        let l2 = remove_element_skip(&mut a2, 3);
        assert_eq!(2, l2);
        assert_eq!(vec![2, 2], &a2[0..l2 as usize]);
    }

    #[test]
    fn test_p27_swap() {
        let mut a1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let l = remove_element_swap(&mut a1, 2);
        assert_eq!(5, l);
        assert_eq!(vec![0, 1, 4, 0, 3], &a1[0..l as usize]); // order doesn't matters

        let mut a2 = vec![3, 2, 2, 3];
        let l2 = remove_element_swap(&mut a2, 3);
        assert_eq!(2, l2);
        assert_eq!(vec![2, 2], &a2[0..l2 as usize]);
    }
}
