// https://leetcode-cn.com/problems/minimum-increment-to-make-array-unique/

pub fn min_increment_for_unique(a: Vec<i32>) -> i32 {
    let mut a = a;
    a.sort_unstable(); // 升序排列
    let l = a.len();
    if l <= 1 {
        return 0;
    }
    let mut count = 0;

    for i in 0..l - 1 {
        let j = i + 1;
        if a[i] == a[j] {
            for k in j..l {
                if a[k] == a[i] {
                    a[k] += 1;
                    count += 1;
                } else {
                    break;
                }
            }
        }
        // println!("{:?}", a);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p945() {
        assert_eq!(0, min_increment_for_unique(vec![]));
        assert_eq!(0, min_increment_for_unique(vec![0]));
        assert_eq!(1, min_increment_for_unique(vec![1, 2, 2]));
        assert_eq!(0, min_increment_for_unique(vec![1, 2]));
        assert_eq!(1, min_increment_for_unique(vec![2, 2]));
        assert_eq!(6, min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]));
    }
}
