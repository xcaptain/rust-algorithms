pub fn solution_of_p44() -> usize {
    let len = 3000;
    let pentagon_arr = gen_pentagon(len);
    let mut min_distance = 999_999_999;
    for i in 1..len - 1 {
        for j in i + 1..len {
            let add_value = pentagon_arr[j] + pentagon_arr[i];
            let sub_value = pentagon_arr[j] - pentagon_arr[i];
            if is_pentagon(sub_value) && is_pentagon(add_value) && sub_value < min_distance {
                min_distance = sub_value;
            }
        }
    }
    min_distance
}

fn gen_pentagon(n: usize) -> Vec<usize> {
    let mut arr = vec![];
    for i in 1..=n {
        arr.push(pentagon(i));
    }
    arr
}

/// p(i) = i * (3 * i - 1) / 2 ==> 3*i^2 - i -2n = 0
/// 1 + (1+24n) / 6
fn pentagon(i: usize) -> usize {
    i * (3 * i - 1) / 2
}

fn is_pentagon(num: usize) -> bool {
    let exp_i = ((1.0 + ((1 + 24 * num) as f64).sqrt()) / 6.0) as usize;
    if pentagon(exp_i) == num {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p44() {
        assert_eq!(5482660, solution_of_p44());
    }

    #[test]
    fn test_is_pentagon() {
        assert_eq!(true, is_pentagon(92));
    }
}
