pub fn solution_of_p32() -> usize {
    let mut v = vec![];
    for i in 1..9999 {
        for j in i + 1..10000 {
            if is_pandigital_product(i, j) {
                v.push(i * j);
                // println!("{}*{}={}", i, j, i*j);
            }
        }
    }
    v.sort();
    v.dedup();
    let mut sum = 0;
    for item in &v {
        sum += item;
    }
    sum
}

fn is_pandigital_product(n1: usize, n2: usize) -> bool {
    let p = n1 * n2;
    let whole_string = format!("{}{}{}", n1, n2, p);
    if whole_string.len() != 9 {
        return false;
    }
    let mut vec_of_string: Vec<char> = whole_string.chars().collect();
    vec_of_string.sort();

    vec_of_string == vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p32() {
        assert_eq!(45228, solution_of_p32());
    }

    #[test]
    fn test_is_pandigital_product() {
        assert_eq!(true, is_pandigital_product(39, 186));
        assert_eq!(false, is_pandigital_product(1, 2));
    }
}
