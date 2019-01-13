use algorithms::misc::permutation::permutation;

/// only 6 numbers satisfy this condition
pub fn solution_of_p41() -> usize {
    let ss = permutation("1234567890");
    let mut sum = 0;
    for s in &ss {
        if &s[0..0] == "0" {
            continue;
        }
        let n = &s.parse::<usize>().unwrap();
        let p1 = (n / 1_000_000) - (n / 1_000_000_000 * 1000);
        // let p1 = &s[1..=3].parse::<usize>().unwrap();
        if p1 % 2 != 0 {
            continue;
        }
        let p2 = (n / 100_000) - (n / 100_000_000 * 1000);
        // let p2 = &s[2..=4].parse::<usize>().unwrap();
        if p2 % 3 != 0 {
            continue;
        }
        let p3 = (n / 10_000) - (n / 10_000_000 * 1000);
        // let p3 = &s[3..=5].parse::<usize>().unwrap();
        if p3 % 5 != 0 {
            continue;
        }
        let p4 = (n / 1000) - (n / 1_000_000 * 1000);
        // let p4 = &s[4..=6].parse::<usize>().unwrap();
        if p4 % 7 != 0 {
            continue;
        }
        let p5 = (n / 100) - (n / 100_000 * 1000);
        // let p5 = &s[5..=7].parse::<usize>().unwrap();
        if p5 % 11 != 0 {
            continue;
        }
        let p6 = (n / 10) - (n / 10_000 * 1000);
        // let p6 = &s[6..=8].parse::<usize>().unwrap();
        if p6 % 13 != 0 {
            continue;
        }
        let p7 = n - (n / 1000 * 1000);
        // let p7 = &s[7..=9].parse::<usize>().unwrap();
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
    sum
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
