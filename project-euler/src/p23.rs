/// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
/// For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
/// A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
/// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24.
/// By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers.
/// However, this upper limit cannot be reduced any further by analysis even though it is known that
/// the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
/// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

pub fn solution() -> usize {
    let mut abundant_arr = vec![];
    let threshold = 28123;
    for i in 12..=threshold {
        if is_abundant(i) {
            abundant_arr.push(i);
        }
    }
    let abundant_arr_len = abundant_arr.len();
    let mut target_arr: Vec<usize> = (0..=threshold).collect();
    for i in 0..abundant_arr_len {
        for j in i..abundant_arr_len {
            let s = abundant_arr[i] + abundant_arr[j];
            if s <= threshold {
                target_arr[s] = 0; // array index is much faster then array search
            }
        }
    }
    target_arr.iter().sum()
}

fn is_abundant(num: usize) -> bool {
    let mut divisor_sum = 1;
    let mid = (num as f64).sqrt() as usize + 1;
    for i in 2..=mid {
        if num % i == 0 {
            let q = num / i;
            if i < q {
                divisor_sum += i;
                divisor_sum += q;
            }
            if i == q {
                divisor_sum += i;
            }
        }
    }
    if divisor_sum > num {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abundant() {
        for i in 1..12 {
            assert_eq!(false, is_abundant(i));
        }
        assert_eq!(true, is_abundant(12));
        assert_eq!(false, is_abundant(13));
        assert_eq!(false, is_abundant(16));
        assert_eq!(true, is_abundant(4088));
        assert_eq!(true, is_abundant(20));
        assert_eq!(false, is_abundant(28)); // 28 is perfect number, so not abundant number
    }

    #[test]
    fn test_solution() {
        assert_eq!(4179871, solution());
    }
}
