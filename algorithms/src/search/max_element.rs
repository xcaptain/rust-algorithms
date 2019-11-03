/// search max element in an array using devide and conquer

pub fn max_element(a: Vec<isize>) -> isize {
    if a.is_empty() {
        return 0;
    }
    return max_element_helper(&a, 0, a.len());
}

/// search max element in a [left, right), because right parenthes is open
/// so use right - left == 1 to determine 1 element array
fn max_element_helper(a: &Vec<isize>, left: usize, right: usize) -> isize {
    if right - left == 1 {
        return a[left];
    }
    let middle = (left + right) / 2;
    let m1 = max_element_helper(a, left, middle);
    let m2 = max_element_helper(a, middle, right);
    if m1 > m2 {
        return m1;
    }
    return m2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_element() {
        assert_eq!(3, max_element(vec![1, 2, 3]));
        assert_eq!(4, max_element(vec![1, 2, 3, 3, 4]));
    }
}
