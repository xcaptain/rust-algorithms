// https://leetcode-cn.com/problems/queries-on-a-permutation-with-key/

pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut p: Vec<i32> = (1..=m).collect();

    for i in 0..queries.len() {
        if let Some(pos) = p.iter().position(|&x| x == queries[i]) {
            res.push(pos as i32);
            // 更新p
            let t = p[pos];
            for j in (1..=pos).rev() {
                p[j] = p[j - 1];
            }
            p[0] = t;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5381() {
        assert_eq!(vec![2, 1, 2, 1], process_queries(vec![3, 1, 2, 1], 5));
        assert_eq!(vec![3, 1, 2, 0], process_queries(vec![4, 1, 2, 2], 4));
        assert_eq!(vec![6, 5, 0, 7, 5], process_queries(vec![7, 5, 5, 8, 3], 8));
    }
}
