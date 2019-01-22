use algorithms::math::lcm::lcm2;
use num_bigint::{BigInt, ToBigInt};

pub fn solution_of_p57() -> usize {
    let mut s = 0;
    let mut curr_frac = continued_fraction(0);
    for _i in 1..=1000 {
        curr_frac = next_continued_fraction(&curr_frac);
        let f_num = curr_frac.numerator.to_string().len();
        let f_deno = curr_frac.denominator.to_string().len();
        if f_num > f_deno {
            s += 1;
        }
    }
    return s;
}

fn next_continued_fraction(curr_frac: &Fraction) -> Fraction {
    let inner_deno = continued_fraction(0).add(curr_frac.clone());
    let inner_frac = continued_fraction(0).add(inner_deno.reciprocal());
    return inner_frac;
}

fn continued_fraction(n: usize) -> Fraction {
    if n == 0 {
        return Fraction::new(BigInt::from(1), BigInt::from(1));
    }
    let mut i = 0;
    let mut result = Fraction::new(BigInt::from(1), BigInt::from(1));
    while i < n {
        let inner_deno = continued_fraction(0).add(result.clone());
        let inner_frac = continued_fraction(0).add(inner_deno.reciprocal());
        result = inner_frac;
        i += 1;
    }
    return result;
}

#[derive(Clone)]
pub struct Fraction {
    pub numerator: BigInt,
    pub denominator: BigInt,
}

impl Fraction {
    pub fn new(numerator: BigInt, denominator: BigInt) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn add(&self, adder: Fraction) -> Fraction {
        let new_deno = lcm2(
            self.denominator.to_bigint().unwrap(),
            adder.denominator.to_bigint().unwrap(),
        );
        let new_self_num = (new_deno.clone() / self.denominator.clone()) * self.numerator.clone();
        let new_adder_num = (new_deno.clone() / adder.denominator) * adder.numerator;
        let new_num = new_self_num + new_adder_num;
        Fraction {
            numerator: new_num,
            denominator: new_deno,
        }
    }

    pub fn reciprocal(&self) -> Fraction {
        Fraction {
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p57() {
        assert_eq!(153, solution_of_p57());
    }
}
