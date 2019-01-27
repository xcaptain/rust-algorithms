use algorithms::math::fraction::Fraction;
use algorithms::math::prime::get_uniq_prime_factors;

pub fn solution_of_p69() -> usize {
    return get_max_n_phi(1_000_000);
}

fn get_max_n_phi(n: usize) -> usize {
    let mut largest_nphi: f64 = 0.0;
    let mut largest_nphi_n = 0;
    for i in 2..n {
        let phi = get_phi(i);
        let n_phi: f64 = (i as f64) / (phi as f64);
        if n_phi > largest_nphi {
            largest_nphi = n_phi;
            largest_nphi_n = i;
        }
    }
    return largest_nphi_n;
}

/// 见欧拉乘积公式：https://en.wikipedia.org/wiki/Euler%27s_totient_function
pub fn get_phi(n: usize) -> usize {
    let factors = get_uniq_prime_factors(n);
    let mut result = Fraction::new(n, 1);
    for f in &factors {
        let p = Fraction::new(1, 1).sub(&Fraction::new(1, *f));
        result = result.mul(&p);
    }

    result = result.reduce();
    return result.numerator;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p69() {
        assert_eq!(510510, solution_of_p69());
    }

    #[test]
    fn test_get_max_n_phi() {
        assert_eq!(6, get_max_n_phi(10));
    }

    #[test]
    fn test_get_phi() {
        assert_eq!(1, get_phi(2));
        assert_eq!(2, get_phi(3));
        assert_eq!(2, get_phi(4));
        assert_eq!(6, get_phi(7));
        assert_eq!(4, get_phi(8));
        assert_eq!(6, get_phi(9));
        assert_eq!(4, get_phi(10));
        assert_eq!(8, get_phi(20));
        assert_eq!(12, get_phi(21));
    }
}
