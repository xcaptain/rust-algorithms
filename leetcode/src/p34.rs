// https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    return search_with_range(&nums, 0, nums.len(), target);
}

fn search_with_range(nums: &Vec<i32>, start: usize, end: usize, target: i32) -> Vec<i32> {
    if end - start == 0 {
        return vec![-1, -1];
    } else if end - start == 1 {
        if nums[start] == target {
            return vec![0, 0];
        }
        return vec![-1, -1];
    } else if end - start == 2 {
        if nums[start] == target && nums[end - 1] != target {
            return vec![start as i32, start as i32];
        } else if nums[start] != target && nums[end - 1] == target {
            return vec![(end - 1) as i32, (end - 1) as i32];
        } else if nums[start] == target && nums[end - 1] == target {
            return vec![start as i32, (end - 1) as i32];
        }
        return vec![-1, -1];
    }
    let mid = (start + end) / 2;
    if nums[mid] < target {
        // 往右边查找
        return search_with_range(nums, mid, end, target);
    } else if nums[mid] > target {
        // 往左边找
        return search_with_range(nums, start, mid, target);
    } else if nums[mid] == target {
        // 在中间，从中间往两边找
        let mut i = mid;
        let mut j = mid;
        while i > start && nums[i - 1] == target {
            i -= 1;
        }

        while j + 1 < end && nums[j + 1] == target {
            j += 1;
        }
        return vec![i as i32, j as i32];
    }

    return vec![-1, -1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p34() {
        // assert_eq!(vec![3, 4], search_range(vec![5, 7, 7, 8, 8, 10], 8));
        // assert_eq!(vec![-1, -1], search_range(vec![5, 7, 7, 8, 8, 10], 6));
        // assert_eq!(vec![-1, -1], search_range(vec![], 6));
        // assert_eq!(vec![-1, -1], search_range(vec![1], 6));
        // assert_eq!(vec![0, 0], search_range(vec![6], 6));
        // assert_eq!(vec![-1, -1], search_range(vec![2, 2], 1));
        assert_eq!(vec![1, 1], search_range(vec![1, 4], 4));
    }
}
