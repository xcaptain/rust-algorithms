pub fn solution_of_p38() -> usize {
    let mut num = 1;
    let mut largest_pandigital = 0;
    // if num > 10000, when i = 1, p has 5 digits, when i = 2,
    // p have over 10 digits, so num must less than 10000
    while num < 10000 {
        let mut s = String::from("");
        for i in 1..=9 {
            let p = num * i;
            s.push_str(&p.to_string());
            if s.len() > 9 {
                break;
            }
            if is_pandigital(&s) {
                let pandigital_num = s.parse::<usize>().unwrap();
                if pandigital_num > largest_pandigital {
                    largest_pandigital = pandigital_num;
                    // println!("largest={}, num={}, i={}", pandigital_num, num, i);
                }
            }
        }
        num += 1;
    }

    largest_pandigital
}

fn is_pandigital(s: &str) -> bool {
    let mut vec_of_string: Vec<char> = s.chars().collect();
    vec_of_string.sort();

    vec_of_string == vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_of_p38() {
        assert_eq!(932718654, solution_of_p38());
    }
}
