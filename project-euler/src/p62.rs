use std::collections::HashMap;

pub fn solution_of_p62() -> usize {
    let cbrt_arr = gen_cubic_arr(100_000_000_000, 1_000_000_000_000);
    // let cbrt_arr = vec![41063625, 56623104, 66430125];
    let mut h: HashMap<String, Vec<usize>> = HashMap::new();
    let l = cbrt_arr.len();
    for item in cbrt_arr.iter().take(l) {
        let mut kv: Vec<char> = item.to_string().chars().collect();
        kv.sort();
        let key: String = kv.into_iter().collect();
        match h.get_mut(&key) {
            Some(v) => {
                let mut new_v: Vec<usize> = v.clone();
                new_v.push(*item);
                *v = new_v;
            }
            None => {
                let new_v = vec![*item];
                h.insert(key, new_v);
            }
        }
    }
    let mut min_value = 1_000_000_000_000;
    for (_k, v) in h.iter() {
        if v.len() == 5 && v[0] < min_value {
            min_value = v[0];
        }
    }
    min_value
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p62() {
        assert_eq!(127035954683, solution_of_p62());
    }
}
