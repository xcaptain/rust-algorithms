pub fn merge_arrays(arr: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    for item in &arr {
        res = merge_2_arrays(res, item.to_owned());
    }
    res
}

fn merge_2_arrays(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut arr = vec![];
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            arr.push(a[i]);
            i += 1;
        } else {
            arr.push(b[j]);
            j += 1;
        }
    }
    for item in a.iter().skip(i) {
        arr.push(item.to_owned());
    }
    for item in b.iter().skip(j) {
        arr.push(item.to_owned());
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_arrays() {
        assert_eq!(
            vec![1, 1, 2, 3, 4, 8, 9],
            merge_arrays(vec![vec![1, 2, 3, 9], vec![1, 4, 8],])
        );
    }
}
