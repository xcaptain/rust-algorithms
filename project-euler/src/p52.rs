pub fn solution_of_p52() -> usize {
    for i in 1..1_000_000 {
        let p2 = 2 * i;
        let p3 = 3 * i;
        let p4 = 4 * i;
        let p5 = 5 * i;
        let p6 = 6 * i;
        if is_contains_same_digits(i, p2)
            && is_contains_same_digits(p3, p4)
            && is_contains_same_digits(p5, p6)
            && is_contains_same_digits(i, p3)
            && is_contains_same_digits(i, p5)
        {
            return i;
        }
    }
    0
}

pub fn is_contains_same_digits(n1: usize, n2: usize) -> bool {
    let s1 = n1.to_string();
    let mut v1: Vec<char> = s1.chars().collect();
    v1.sort();
    let s2 = n2.to_string();
    let mut v2: Vec<char> = s2.chars().collect();
    v2.sort();

    v1 == v2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p52() {
        assert_eq!(142857, solution_of_p52());
    }

    #[test]
    fn test_is_contains_same_digits() {
        assert_eq!(true, is_contains_same_digits(123, 321));
        assert_eq!(false, is_contains_same_digits(123, 3211));
        assert_eq!(false, is_contains_same_digits(123, 3210));
    }
}
