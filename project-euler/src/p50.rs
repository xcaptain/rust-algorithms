use algorithms::math::prime::{get_prime_table_le, is_prime};

pub fn solution_of_p50(uplimit: usize) -> usize {
    let primes = get_prime_table_le(uplimit);
    let l = primes.len();
    let mut longest_seq_sum = 0;
    let mut longest_seq = 0;

    for i in 0..l - 1 {
        let mut sum = primes[i];
        for (j, item) in primes.iter().enumerate().take(l).skip(i + 1) {
            sum += item;
            if sum > uplimit {
                break;
            }
            if is_prime(sum) && longest_seq < j - i {
                longest_seq = j - i;
                longest_seq_sum = sum;
            }
        }
    }
    longest_seq_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p50() {
        assert_eq!(41, solution_of_p50(100));
        assert_eq!(997651, solution_of_p50(1_000_000));
    }
}
