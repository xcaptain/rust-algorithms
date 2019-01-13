use algorithms::math::prime::get_uniq_prime_factors;

pub fn solution_of_p47() -> usize {
    let mut i = 2;
    loop {
        let factors1 = get_uniq_prime_factors(i);
        let factors2 = get_uniq_prime_factors(i + 1);
        let factors3 = get_uniq_prime_factors(i + 2);
        let factors4 = get_uniq_prime_factors(i + 3);

        if factors1.len() == 4 && factors2.len() == 4 && factors3.len() == 4 && factors4.len() == 4
        {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p47() {
        assert_eq!(134043, solution_of_p47());
    }
}
