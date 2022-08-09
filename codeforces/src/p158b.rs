// https://codeforces.com/problemset/problem/158/B

use crate::{Scanner, Solution};
use std::collections::HashMap;
use std::io::{Read, Write};

pub struct Solution1;

impl Solution for Solution1 {
    fn solve(&self, input: &mut dyn Read, output: &mut dyn Write) {
        let mut scanner = Scanner::new(input);
        let _n = scanner.next_line::<String>().parse::<usize>().unwrap();
        let arr: Vec<usize> = scanner
            .next_line::<String>()
            .split(' ')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();

        let mut groups: HashMap<usize, usize> = HashMap::new();
        groups.insert(4, 0);
        groups.insert(3, 0);
        groups.insert(2, 0);
        groups.insert(1, 0);
        let mut res = 0;
        for (_i, item) in arr.iter().enumerate() {
            if let Some(n) = groups.get_mut(item) {
                (*n) += 1;
            }
        }
        let mut one_num = *(groups.get(&1).unwrap());
        let two_num = *(groups.get(&2).unwrap());
        let mut three_num = *(groups.get(&3).unwrap());
        let four_num = *(groups.get(&4).unwrap());
        res += four_num + two_num / 2;
        // 尽量把3和1凑一起
        if three_num > 0 && one_num > 0 {
            if three_num > one_num {
                res += one_num;
                three_num -= one_num;
                one_num = 0;
            } else {
                res += three_num;
                one_num -= three_num;
                three_num = 0;
            }
        }
        if two_num % 2 == 0 {
            // two has been matched
            if one_num > 0 {
                // this group only contains 1
                res += one_num / 4;
                let r = one_num % 4;
                if r > 0 {
                    res += 1;
                }
            } else if three_num > 0 {
                // this group only contains 3
                res += three_num;
            }
        } else if three_num > 0 {
            res += three_num;
            res += 1;
        } else if one_num > 0 {
            // this group only contains 1
            res += one_num / 4;
            let r = one_num % 4;
            if r > 0 {
                if r <= 2 {
                    res += 1;
                } else {
                    res += 2; // the remaining 2 must be a group
                }
            } else {
                res += 1;
            }
        } else {
            res += 1;
        }
        write!(output, "{}", res).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper;

    #[test]
    fn solution1() {
        let cases = vec![
            [
                "5
1 2 4 3 3",
                "4",
            ],
            [
                "8
2 3 4 4 2 1 3 1",
                "5",
            ],
            [
                "4
2 4 1 3",
                "3",
            ],
            [
                "2
1 1", "1",
            ],
        ];
        test_helper(cases, Solution1);
    }
}
