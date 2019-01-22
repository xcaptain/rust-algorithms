use crate::math::lcm::lcm2;
use num_bigint::{BigInt, ToBigInt};

#[derive(Clone, Debug)]
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

    pub fn add(&self, adder: &Fraction) -> Fraction {
        let new_deno = lcm2(
            self.denominator.to_bigint().unwrap(),
            adder.denominator.to_bigint().unwrap(),
        );
        let new_self_num = (new_deno.clone() / self.denominator.clone()) * self.numerator.clone();
        let new_adder_num =
            (new_deno.clone() / adder.denominator.clone()) * adder.numerator.clone();
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
    fn test_clone() {
        let f1 = Fraction::new(BigInt::from(1), BigInt::from(2));
        let f2 = f1.clone();

        assert_eq!(BigInt::from(1), f2.numerator);
    }

    #[test]
    fn test_add() {
        let f1 = Fraction::new(BigInt::from(1), BigInt::from(2));
        let f2 = f1.add(&f1);

        assert_eq!(BigInt::from(2), f2.numerator);
        assert_eq!(BigInt::from(2), f2.denominator);
    }
}
