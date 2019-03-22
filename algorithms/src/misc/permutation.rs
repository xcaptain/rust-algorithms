pub fn permutation(s: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    permutate("", s, &mut result);
    result
}

fn permutate(prefix: &str, s: &str, result: &mut Vec<String>) {
    let n = s.len();
    if n == 0 {
        result.push(prefix.to_owned());
    } else {
        for i in 0..n {
            let s1 = format!("{}{}", prefix.to_owned(), &s[i..=i].to_owned());
            let s2 = format!("{}{}", s[0..i].to_owned(), s[i + 1..n].to_owned());
            permutate(s1.as_str(), s2.as_str(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation() {
        let a = permutation("1234");
        assert_eq!(24, a.len());
    }
}
