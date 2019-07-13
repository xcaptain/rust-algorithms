// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/

use std::cmp::max;

// brute force solution, very very slow
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }

    let mut profit = 0;
    for i in 0..prices.len() - 1 {
        for j in i + 1..prices.len() {
            if prices[j] > prices[i] {
                profit = max(profit, prices[j] - prices[i]);
            }
        }
    }
    return profit;
}

pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }

    let mut profit = 0;
    let mut in_pos = 0;
    for i in 0..prices.len() {
        profit = max(profit, prices[i] - prices[in_pos]);
        if prices[i] < prices[in_pos] {
            in_pos = i;
        }
    }
    return profit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(5, max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, max_profit(vec![7, 6, 4, 3, 1]));

        assert_eq!(5, max_profit_v2(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, max_profit_v2(vec![7, 6, 4, 3, 1]));
    }
}
