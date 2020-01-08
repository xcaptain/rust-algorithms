// https://leetcode-cn.com/problems/sort-colors/

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut zero_count = 0_usize;
    let mut one_count = 0_usize;
    let mut two_count = 0_usize;

    for num in nums.iter() {
        if *num == 0 {
            zero_count += 1;
        } else if *num == 1 {
            one_count += 1;
        } else if *num == 2 {
            two_count += 1;
        }
    }

    nums.splice(0..zero_count, vec![0; zero_count]);
    nums.splice(zero_count..zero_count + one_count, vec![1; one_count]);
    nums.splice(
        zero_count + one_count..zero_count + one_count + two_count,
        vec![2; two_count],
    );
}

pub fn sort_colors_with_pointer(nums: &mut Vec<i32>) {
    let mut p0 = 0_usize; // 0的右边界
    let mut cur = 0_usize; // 当前遍历位置
    let mut p2 = nums.len() - 1; // 2的左边界

    while cur <= p2 {
        if nums[cur] == 0 {
            // p0 右移并且交换
            nums.swap(cur, p0);
            p0 += 1;
            cur += 1;
        } else if nums[cur] == 2 {
            // p2 左移
            nums.swap(cur, p2);
            if p2 == 0 {
                break;
            }
            p2 -= 1;
        } else {
            cur += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p75() {
        let mut arr1 = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut arr1);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], arr1);

        let mut arr2 = vec![0];
        sort_colors(&mut arr2);
        assert_eq!(vec![0], arr2);
    }

    #[test]
    fn test_p75_using_pointer() {
        let mut arr1 = vec![2, 0, 2, 1, 1, 0];
        sort_colors_with_pointer(&mut arr1);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], arr1);

        let mut arr2 = vec![2];
        sort_colors_with_pointer(&mut arr2);
        assert_eq!(vec![2], arr2);
    }
}
