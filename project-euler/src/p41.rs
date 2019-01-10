use algorithms::math::is_prime::is_prime;
use algorithms::misc::permutation::permutation;

pub fn largest_pandigital_prime() -> usize {
    let ns = permutation("1234567");
    let mut largest = 0;
    for val in &ns {
        let num = val.parse::<usize>().unwrap();
        if is_prime(num) && num > largest {
            largest = num;
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_pandigital_prime() {
        assert_eq!(7652413, largest_pandigital_prime());
    }
}
