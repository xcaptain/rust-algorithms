// https://leetcode-cn.com/contest/weekly-contest-177/problems/closest-divisors/

pub fn closest_divisors(num: i32) -> Vec<i32> {
    let mut divisors = get_divisor_pairs(num);
    divisors.sort_by(|a, b| a[2].cmp(&b[2]));
    // println!("divisors: {:?}", divisors);
    vec![divisors[0][0], divisors[0][1]]
}

fn get_divisor_pairs(num: i32) -> Vec<[i32; 3]> {
    let mut res = vec![];
    let m1 = ((num + 1) as f64).sqrt() as i32;
    for i in 1..=m1 {
        let q1 = (num + 1) / i;
        let r1 = (num + 1) % i;
        if r1 == 0 {
            res.push([i, q1, q1 - i]);
        }
    }

    let m2 = ((num + 2) as f64).sqrt() as i32;
    for i in 1..=m2 {
        let q1 = (num + 2) / i;
        let r1 = (num + 2) % i;
        if r1 == 0 {
            res.push([i, q1, q1 - i]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5171() {
        assert_eq!(vec![3, 3], closest_divisors(8));
        assert_eq!(vec![5, 25], closest_divisors(123));
        assert_eq!(vec![25, 40], closest_divisors(999));
        assert_eq!(vec![1, 2], closest_divisors(1));
        assert_eq!(vec![2, 2], closest_divisors(2));
    }
}
