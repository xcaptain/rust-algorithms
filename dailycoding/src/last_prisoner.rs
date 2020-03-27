//! This problem was asked by Bloomberg.
//! There are N prisoners standing in a circle, waiting to be executed. The executions are carried out starting with the kth person,
//!  and removing every successive kth person going clockwise until there is no one left.
//! Given N and k, write an algorithm to determine where a prisoner should stand in order to be the last survivor.
//! For example, if N = 5 and k = 2, the order of executions would be [2, 4, 1, 5, 3], so you should return 3.
//! Bonus: Find an O(log N) solution if k = 2.
//!
//! solutions inlude:
//! 1. loop linked list
//! 2. array
//! 3. recursive function

pub fn get_last_prisoner(n: usize, k: usize) -> usize {
    let mut flag = vec![1; n];

    let mut out_cnt = 0;
    let mut num_off = 0;
    while out_cnt < n - 1 {
        for i in 0..n {
            if flag[i] == 1 {
                num_off += 1;
                // 如果达到了最后一个位置，这时num_off还不到k，通过while
                // 循环再从头for循环一遍
                if num_off == k {
                    out_cnt += 1;
                    flag[i] = 0;
                    num_off = 0;
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..n {
        if flag[i] == 1 {
            res = i + 1;
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_last_prisoner() {
        assert_eq!(3, get_last_prisoner(5, 2));
    }
}
