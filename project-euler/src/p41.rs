use algorithms::math::is_prime::is_prime;

pub fn permutate(prefix: &str, s: &str, result: &mut Vec<String>) {
    let n = s.len();
    if n == 0 {
        result.push(prefix.to_owned());
    } else {
        for i in 0..n {
            let s1 = format!("{}{}", prefix.to_owned(), &s[i..=i].to_owned());
            let s2 = format!("{}{}", s[0..i].to_owned(), s[i + 1..n].to_owned());
            permutate(s1.as_str(), s2.as_str(), result);
        }
    }
}

pub fn largest_pandigital_prime() -> usize {
    let mut ns = vec![];
    permutate("", "1234567", &mut ns);
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
    fn test_permutate() {
        let mut a = vec![];
        permutate("", "1234", &mut a);
        assert_eq!(24, a.len());
    }

    #[test]
    fn test_largest_pandigital_prime() {
        assert_eq!(7652413, largest_pandigital_prime());
    }
}
