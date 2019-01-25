use algorithms::math::prime::{get_prime_table, is_prime};

pub fn solution_of_p60() -> usize {
    let l = 10000;
    let primes = get_prime_table(l);
    for a in &primes {
        let pca = get_prime_combs(&primes, *a);
        for b in &pca {
            let pcb = get_prime_combs(&pca, *b);
            for c in &pcb {
                let pcc = get_prime_combs(&pcb, *c);
                for d in &pcc {
                    let pcd = get_prime_combs(&pcc, *d);
                    for e in &pcd {
                        println!("{}, {}, {}, {}, {}", a, b, c, d, e);
                    }
                }
            }
        }
    }
    return 1;
}

fn get_prime_combs(primes: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut arr = vec![];
    for p in primes {
        if *p > n && is_valid_prime_pair(*p, n) {
            arr.push(*p);
        }
    }
    return arr;
}

fn is_valid_prime_pair(c: usize, n: usize) -> bool {
    let pair1 = format!("{}{}", n, c).parse::<usize>().unwrap();
    let pair2 = format!("{}{}", c, n).parse::<usize>().unwrap();
    if !is_prime(pair1) || !is_prime(pair2) {
        return false;
    }
    return true;
}

fn validate_prime_pair() -> bool {
    // 3, 3119, 9887, 36263, 48731
    // 3, 5323, 10357, 29587, 31231
    // 3, 22037, 28283, 61463, 100649
    // 3, 28277, 44111, 70241, 78509
    let arr = [3, 28277, 44111, 70241, 78509];
    let l = arr.len();
    for i in 0..l - 1 {
        for j in i + 1..l {
            if !is_valid_prime_pair(arr[i], arr[j]) {
                println!("{}, {}", arr[i], arr[j]);
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p60() {
        assert_eq!(1, solution_of_p60());
    }

    #[test]
    fn test_validate_prime_pair() {
        assert_eq!(true, validate_prime_pair());
    }
}
