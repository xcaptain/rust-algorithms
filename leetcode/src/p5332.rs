// https://leetcode-cn.com/contest/weekly-contest-175/problems/check-if-n-and-its-double-exist/

pub fn check_if_exist(arr: Vec<i32>) -> bool {
    let l = arr.len();
    for i in 0..l - 1 {
        for j in i + 1..l {
            if arr[i] == 2 * arr[j] || arr[j] == 2 * arr[i] {
                return true;
            }
        }
    }
    // for num in &arr {
    //     if num != &0 && (arr.contains(&(num * 2)) || (num % 2 == 0 && arr.contains(&(num / 2)))) {
    //         return true;
    //     }
    // }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5332() {
        assert_eq!(true, check_if_exist(vec![10, 2, 5, 3]));
        assert_eq!(true, check_if_exist(vec![7, 1, 14, 11]));
        assert_eq!(true, check_if_exist(vec![0, 0]));
        assert_eq!(false, check_if_exist(vec![0]));
        assert_eq!(false, check_if_exist(vec![3, 1, 7, 11]));
        assert_eq!(false, check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8]));
    }
}
