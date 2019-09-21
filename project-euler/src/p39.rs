pub fn solution_of_p39() -> usize {
    let mut max_num = 0;
    let mut max_value = 0;
    for p in 4..1000 {
        let num = get_right_triangles(p);
        if num > max_num {
            max_num = num;
            max_value = p;
        }
    }
    max_value
}

fn get_right_triangles(p: usize) -> usize {
    let mut sum = 0;
    for a in 1..p / 2 {
        for b in a + 1..=(p - a) / 2 {
            let c = p - a - b;
            if a * a + b * b == c * c {
                sum += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p39() {
        assert_eq!(840, solution_of_p39());
    }

    #[test]
    fn test_get_right_triangles() {
        assert_eq!(3, get_right_triangles(120));
    }
}
