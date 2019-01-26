use algorithms::math::fraction::Fraction;
use num_bigint::BigInt;

pub fn solution_of_p65() -> usize {
    return sum_of_convergent(100, gen_e_seq(100));
}

fn sum_of_convergent(n: usize, arr: Vec<usize>) -> usize {
    let nth_convergent_of_e = convergent(n, arr);
    let mut s = 0;
    for n in nth_convergent_of_e.numerator.to_string().chars() {
        let num = n.to_string().parse::<usize>().unwrap();
        s += num;
    }
    return s;
}

pub fn convergent(n: usize, arr: Vec<usize>) -> Fraction<BigInt> {
    let a0 = Fraction::new(BigInt::from(arr[0]), BigInt::from(1));
    if n == 1 {
        return a0;
    }

    let mut cur_index = n - 1;
    let mut result = Fraction::new(BigInt::from(1), BigInt::from(arr[cur_index]));
    loop {
        let prev_index = cur_index - 1;
        result = Fraction::new(BigInt::from(arr[prev_index]), BigInt::from(1)).add(&result);
        if prev_index == 0 {
            return result;
        }
        result = result.reciprocal().reduce();
        cur_index = prev_index;
    }
}

fn gen_e_seq(n: usize) -> Vec<usize> {
    let mut i = 0;
    let mut arr = vec![2];
    let mut k = 0;
    while i < n {
        if i % 3 == 0 {
            arr.push(1);
        } else if i % 3 == 1 {
            arr.push(2 * (k + 1));
            k += 1;
        } else {
            arr.push(1);
        }
        i += 1;
    }
    return arr;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p65() {
        assert_eq!(272, solution_of_p65());
    }

    #[test]
    fn test_sum_of_convergent() {
        assert_eq!(13, sum_of_convergent(9, gen_e_seq(100)));
        assert_eq!(17, sum_of_convergent(10, gen_e_seq(100)));
    }

    #[test]
    fn test_gen_e_seq() {
        assert_eq!(vec![2, 1, 2, 1], gen_e_seq(3));
        assert_eq!(vec![2, 1, 2, 1, 1, 4, 1], gen_e_seq(6));
    }

    #[test]
    fn test_convergent() {
        let f1 = convergent(1, gen_e_seq(100));
        assert_eq!(BigInt::from(2), f1.numerator);
        assert_eq!(BigInt::from(1), f1.denominator);

        let f2 = convergent(2, gen_e_seq(100));
        assert_eq!(BigInt::from(3), f2.numerator);
        assert_eq!(BigInt::from(1), f2.denominator);

        let f3 = convergent(3, gen_e_seq(100));
        assert_eq!(BigInt::from(8), f3.numerator);
        assert_eq!(BigInt::from(3), f3.denominator);

        let f4 = convergent(4, gen_e_seq(100));
        assert_eq!(BigInt::from(11), f4.numerator);
        assert_eq!(BigInt::from(4), f4.denominator);

        let f10 = convergent(10, gen_e_seq(100));
        assert_eq!(BigInt::from(1457), f10.numerator);
        assert_eq!(BigInt::from(536), f10.denominator);
    }
}
