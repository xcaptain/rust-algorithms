/// see http://mathworld.wolfram.com/HexagonalPentagonalNumber.html
/// https://projecteuler.net/problem=45

pub fn solution_of_p45() -> usize {
    let mut i = 40756;
    loop {
        if is_pentagon(i) && is_triangle(i) && is_hexagonal(i) {
            return i;
        }
        i += 1;
    }
}

fn is_pentagon(num: usize) -> bool {
    let exp_i = ((1.0 + ((1 + 24 * num) as f64).sqrt()) / 6.0) as usize;
    if pentagon(exp_i) == num {
        return true;
    }
    false
}

// x^2 + x -2n = 0;
// (-1 + (1 + 8n).sqrt) /2
fn is_triangle(num: usize) -> bool {
    let exp_i = ((((1 + 8 * num) as f64).sqrt() - 1.0) / 2.0) as usize;
    if triangle(exp_i) == num {
        return true;
    }
    false
}

// 2x^2 - x - n = 0
// 1 + (1 + 8n).sqrt / 4
fn is_hexagonal(num: usize) -> bool {
    let exp_i = ((((1 + 8 * num) as f64).sqrt() + 1.0) / 4.0) as usize;
    if hexagonal(exp_i) == num {
        return true;
    }
    false
}

fn pentagon(i: usize) -> usize {
    i * (3 * i - 1) / 2
}

fn triangle(i: usize) -> usize {
    i * (i + 1) / 2
}

fn hexagonal(i: usize) -> usize {
    i * (2 * i - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p45() {
        assert_eq!(1533776805, solution_of_p45());
    }
}
