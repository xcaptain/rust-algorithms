// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii/
use std::cmp::max;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut total_profit = 0;
    for i in 1..prices.len() {
        total_profit += max(0, prices[i] - prices[i - 1]);
    }
    return total_profit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(7, max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(4, max_profit(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, max_profit(vec![7, 6, 4, 3, 1]));
        assert_eq!(7, max_profit(vec![6, 1, 3, 2, 4, 7]));
    }
}
