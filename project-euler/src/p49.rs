use algorithms::math::prime::is_prime;
use algorithms::misc::permutation::permutation;

pub fn solution_of_p49() -> Vec<String> {
    let mut arr: Vec<usize> = vec![];
    let mut result: Vec<String> = vec![];
    for i in 1000..9999 {
        if is_prime(i) {
            let permutations = permutation(&i.to_string()[..]);
            arr.push(i);
            for perm in &permutations {
                let num = perm.parse::<usize>().unwrap();
                if num < 1000 {
                    continue;
                }
                if num != i && is_prime(num) {
                    arr.push(num);
                }
            }
            arr.sort();
            arr.dedup();
            if arr.len() >= 3 {
                let l = arr.len();
                for ii in 0..l - 2 {
                    for jj in ii + 1..l - 1 {
                        for kk in jj + 1..l {
                            if arr[ii] + arr[kk] == 2 * arr[jj] {
                                result.push(format!("{}{}{}", arr[ii], arr[jj], arr[kk]));
                            }
                        }
                    }
                }
            }
            arr = vec![];
        }
    }
    result.sort();
    result.dedup();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p49() {
        assert_eq!(vec!["148748178147", "296962999629"], solution_of_p49());
    }
}
