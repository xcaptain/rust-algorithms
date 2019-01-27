use crate::p69::get_phi;

pub fn solution_of_p70() -> usize {
    let mut min_nphi: f64 = 1000000.0;
    let mut min_nphi_n = 0;
    for i in 2..10_000_000 {
        let phi = get_phi(i);
        if is_permutate(phi, i) {
            let n_phi: f64 = (i as f64) / (phi as f64);
            if n_phi < min_nphi {
                min_nphi = n_phi;
                min_nphi_n = i;
            }
        }
        println!("i={}", i);
    }
    return min_nphi_n;
}

fn is_permutate(a: usize, b: usize) -> bool {
    let mut av: Vec<char> = a.to_string().chars().collect();
    av.sort();
    let mut bv: Vec<char> = b.to_string().chars().collect();
    bv.sort();
    if av != bv {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p70() {
        assert_eq!(8319823, solution_of_p70());
    }
}
