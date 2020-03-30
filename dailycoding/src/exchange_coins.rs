//! This problem was asked by Google.
//! Find the minimum number of coins required to make n cents.
//! You can use standard American denominations, that is, 1¢, 5¢, 10¢, and 25¢.
//! For example, given n = 16, return 3 since we can make it with a 10¢, a 5¢, and a 1¢.

pub fn least_exchange_coins(n: usize) -> usize {
    if n >= 25 {
        return n / 25 + least_exchange_coins(n % 25);
    } else if n >= 10 {
        return n / 10 + least_exchange_coins(n % 10);
    } else if n >= 5 {
        return n / 5 + least_exchange_coins(n % 5);
    } else if n >= 1 {
        return n;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_exchange_coin_method_num() {
        assert_eq!(3, least_exchange_coins(16));
        assert_eq!(6, least_exchange_coins(19));
        assert_eq!(2, least_exchange_coins(20));
        assert_eq!(4, least_exchange_coins(41));
    }
}
