pub fn solution_of_p33() -> usize {
    // let nums = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for numerator in 10..98 {
        for denominator in numerator + 1..99 {
            if numerator % 10 == 0 || denominator % 10 == 0 {
                continue;
            }
            let numerator_string = numerator.to_string();
            let denominator_string = denominator.to_string();

            let n0 = numerator_string.chars().nth(0).unwrap();
            let n1 = numerator_string.chars().nth(1).unwrap();

            let d0 = denominator_string.chars().nth(0).unwrap();
            let d1 = denominator_string.chars().nth(1).unwrap();
            if n0 == d0 {
                let nn = numerator_string[1..].parse::<usize>().unwrap();
                let nd = denominator_string[1..].parse::<usize>().unwrap();
                if nd != 0 && (nd % nn) == (denominator % numerator) {
                    println!("{}, {}", numerator, denominator);
                }
            }
            if n0 == d1 {
                let nn = numerator_string[1..].parse::<usize>().unwrap();
                let nd = denominator_string[0..1].parse::<usize>().unwrap();
                if nd != 0 && (nd % nn) == (denominator % numerator) {
                    println!("{}, {}", numerator, denominator);
                }
            } else if n1 == d0 {
                let nn = numerator_string[0..1].parse::<usize>().unwrap();
                let nd = denominator_string[1..].parse::<usize>().unwrap();
                if nd != 0 && (nd % nn) == (denominator % numerator) {
                    println!("{}, {}", numerator, denominator);
                }
            } else if n1 == d1 {
                let nn = numerator_string[0..1].parse::<usize>().unwrap();
                let nd = denominator_string[0..1].parse::<usize>().unwrap();
                if nd != 0 && (nd % nn) == (denominator % numerator) {
                    println!("{}, {}", numerator, denominator);
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p33() {
        assert_eq!(1, solution_of_p33());
    }
}
