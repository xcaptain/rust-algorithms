pub fn solution_of_p62() -> usize {
    let cbrt_arr = gen_cubic_arr(1_00_000_000_000, 1_000_000_000_000);
    // let cbrt_arr = vec![41063625, 56623104, 66430125];
    let l = cbrt_arr.len();
    for i in 0..l {
        let mut perm_num = 0;
        for j in i + 1..l {
            if is_permutate(cbrt_arr[i], cbrt_arr[j]) {
                perm_num += 1;
            }
        }
        if perm_num == 4 {
            return cbrt_arr[i];
        }
    }
    return 1;
}

fn gen_cubic_arr(start: usize, end: usize) -> Vec<usize> {
    let mut n = (start as f64).cbrt() as usize;
    let mut arr = vec![];
    loop {
        let c = n * n * n;
        if c < end {
            arr.push(c);
            n += 1;
        } else {
            return arr;
        }
    }
}

fn is_permutate(a: usize, b: usize) -> bool {
    let mut astr: Vec<char> = a.to_string().chars().collect();
    astr.sort();
    let mut bstr: Vec<char> = b.to_string().chars().collect();
    bstr.sort();
    if astr != bstr {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p62() {
        assert_eq!(127035954683, solution_of_p62());
    }
}
