use algorithms::math::prime::get_prime_table;

pub fn solution_of_p46() -> usize {
    let primes = get_prime_table(1000); // 前1000个素数
    let mut i = 4;
    loop {
        i += 1;
        let odd_compose = 2 * i + 1;
        if primes.contains(&odd_compose) {
            continue;
        }
        let middle = ((odd_compose / 2) as f64).sqrt() as usize + 1;
        let mut included = false;
        for j in 1..=middle {
            if odd_compose < 2 * j * j {
                break;
            }
            let exp_prime = odd_compose - 2 * j * j;
            if primes.contains(&exp_prime) {
                included = true;
                break;
            }
        }
        if !included {
            return odd_compose;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p46() {
        assert_eq!(5777, solution_of_p46());
    }
}
