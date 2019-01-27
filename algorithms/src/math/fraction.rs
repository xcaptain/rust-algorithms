use num_integer::Integer;

#[derive(Clone, Debug)]
pub struct Fraction<T: Integer + Clone> {
    pub numerator: T,
    pub denominator: T,
}

impl<T: Integer + Clone> Fraction<T> {
    pub fn new(numerator: T, denominator: T) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn add(&self, adder: &Fraction<T>) -> Fraction<T> {
        let new_deno = lcm(self.denominator.clone(), adder.denominator.clone());
        let new_self_num = (new_deno.clone() / self.denominator.clone()) * self.numerator.clone();
        let new_adder_num =
            (new_deno.clone() / adder.denominator.clone()) * adder.numerator.clone();
        let new_num = new_self_num + new_adder_num;
        Fraction {
            numerator: new_num,
            denominator: new_deno,
        }
    }

    pub fn sub(&self, subtractor: &Fraction<T>) -> Fraction<T> {
        let new_deno = lcm(self.denominator.clone(), subtractor.denominator.clone());
        let new_self_num = (new_deno.clone() / self.denominator.clone()) * self.numerator.clone();
        let new_sub_num =
            (new_deno.clone() / subtractor.denominator.clone()) * subtractor.numerator.clone();
        let new_num = new_self_num - new_sub_num;
        Fraction {
            numerator: new_num,
            denominator: new_deno,
        }
    }

    pub fn mul(&self, multiplier: &Fraction<T>) -> Fraction<T> {
        Fraction {
            numerator: self.numerator.clone() * multiplier.numerator.clone(),
            denominator: self.denominator.clone() * multiplier.denominator.clone(),
        }
    }

    pub fn reciprocal(&self) -> Fraction<T> {
        Fraction {
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }

    pub fn reduce(&self) -> Fraction<T> {
        let divisor = gcd(self.numerator.clone(), self.denominator.clone());
        if divisor > T::one() {
            return Fraction {
                numerator: self.numerator.clone() / divisor.clone(),
                denominator: self.denominator.clone() / divisor.clone(),
            };
        }
        return self.clone();
    }
}

pub fn lcm<T: Integer + Clone>(m: T, n: T) -> T {
    let g = gcd(m.clone(), n.clone());
    return m * n / g;
}

pub fn gcd<T: Integer + Clone>(mm: T, nn: T) -> T {
    let mut m = mm;
    let mut n = nn;
    while n != T::zero() {
        let t = n.clone();
        n = m % n;
        m = t;
    }
    return m;
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    #[test]
    fn test_clone() {
        let f1 = Fraction::new(1, 2);
        let f2 = f1.clone();

        assert_eq!(1, f2.numerator);
    }

    #[test]
    fn test_big_int_clone() {
        let f1 = Fraction::new(BigInt::from(1), BigInt::from(2));
        let f2 = f1.clone();

        assert_eq!(BigInt::from(1), f2.numerator);
    }

    #[test]
    fn test_big_int_add() {
        let f1 = Fraction::new(BigInt::from(1), BigInt::from(2));
        let f2 = f1.add(&f1);

        assert_eq!(BigInt::from(2), f2.numerator);
        assert_eq!(BigInt::from(2), f2.denominator);
    }

    #[test]
    fn test_add() {
        let f1 = Fraction::new(1, 2);
        let f2 = f1.add(&f1);

        assert_eq!(2, f2.numerator);
        assert_eq!(2, f2.denominator);
    }

    #[test]
    fn test_sub() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 3);
        let f3 = f1.sub(&f2);
        assert_eq!(1, f3.numerator);
        assert_eq!(6, f3.denominator);
    }

    #[test]
    fn test_mul() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 3);
        let f3 = f1.mul(&f2);
        assert_eq!(1, f3.numerator);
        assert_eq!(6, f3.denominator);
    }
}
