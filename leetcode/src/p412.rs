pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|c| {
            if c % 15 == 0 {
                String::from("FizzBuzz")
            } else if c % 5 == 0 {
                String::from("Buzz")
            } else if c % 3 == 0 {
                String::from("Fizz")
            } else {
                c.to_string()
            }
        })
        .collect()
}

pub fn fizz_buzz_v2(n: i32) -> Vec<String> {
    let mut res = vec![];
    for c in 1..=n {
        if c % 15 == 0 {
            res.push(String::from("FizzBuzz"));
        } else if c % 5 == 0 {
            res.push(String::from("Buzz"));
        } else if c % 3 == 0 {
            res.push(String::from("Fizz"));
        } else {
            res.push(c.to_string());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p412() {
        assert_eq!(
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ],
            fizz_buzz(15)
        );
    }
}
