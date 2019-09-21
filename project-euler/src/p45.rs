/// see http://mathworld.wolfram.com/HexagonalPentagonalNumber.html
/// https://projecteuler.net/problem=45

pub fn solution_of_p45() -> usize {
    let mut i = 1;
    loop {
        i += 1;
        let h = i * (2 * i - 1);
        let k = (1.0 + (24.0 * (h as f64) + 1 as f64).sqrt()) / 6.0;
        let k2 = (k as i64) as f64;
        if k == k2 && h > 40755 {
            return h;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p45() {
        assert_eq!(1533776805, solution_of_p45());
    }
}
