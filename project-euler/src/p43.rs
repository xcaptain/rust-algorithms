use algorithms::misc::permutation::permutation;

/// only 6 numbers satisfy this condition
pub fn solution_of_p41() -> usize {
    let ss = permutation("0123456789");
    let mut sum = 0;
    for s in &ss {
        let p1 = &s[1..=3].parse::<usize>().unwrap();
        if p1 % 2 != 0 {
            continue;
        }
        let p2 = &s[2..=4].parse::<usize>().unwrap();
        if p2 % 3 != 0 {
            continue;
        }
        let p3 = &s[3..=5].parse::<usize>().unwrap();
        if p3 % 5 != 0 {
            continue;
        }
        let p4 = &s[4..=6].parse::<usize>().unwrap();
        if p4 % 7 != 0 {
            continue;
        }
        let p5 = &s[5..=7].parse::<usize>().unwrap();
        if p5 % 11 != 0 {
            continue;
        }
        let p6 = &s[6..=8].parse::<usize>().unwrap();
        if p6 % 13 != 0 {
            continue;
        }
        let p7 = &s[7..=9].parse::<usize>().unwrap();
        if p7 % 17 != 0 {
            continue;
        }

        // println!(
        //     "s: {}, p1: {}, p2: {}, p3: {}, p4: {}, p5: {}, p6: {}, p7: {}",
        //     s, p1, p2, p3, p4, p5, p6, p7
        // );
        let n = &s.parse::<usize>().unwrap();
        sum += n;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p41() {
        let result = solution_of_p41();
        assert_eq!(16695334890, result);
    }
}
