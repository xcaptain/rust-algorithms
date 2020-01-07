use std::collections::HashMap;

pub fn four_sum_count(one: Vec<i32>, two: Vec<i32>, three: Vec<i32>, four: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut m: HashMap<i32, i32> = HashMap::new();
    for aa in &one {
        for bb in &two {
            let v = m.entry(aa + bb).or_insert(0);
            *v += 1;
        }
    }
    for cc in &three {
        for dd in &four {
            if let Some(count) = m.get(&(0 - (cc + dd))) {
                res += count;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p454() {
        assert_eq!(
            2,
            four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2])
        );
    }
}
