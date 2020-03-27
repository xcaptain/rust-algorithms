//! This problem was asked by Jane Street.
//! Given an arithmetic expression in Reverse Polish Notation, write a program to evaluate it.
//! The expression is given as a list of numbers and operands. For example: [5, 3, '+'] should return 5 + 3 = 8.
//! For example,
//! [15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-'] should return 5,
//! since it is equivalent to ((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1)) = 5.
//! You can assume the given expression is always valid.

pub fn calculate(exp: Vec<&str>) -> i32 {
    let mut st = vec![];
    for item in exp {
        if item == "+" || item == "-" || item == "*" || item == "/" {
            let a = st.pop().unwrap(); // close one
            let b = st.pop().unwrap(); // far one
            if item == "+" {
                st.push(b + a);
            } else if item == "-" {
                st.push(b - a);
            } else if item == "*" {
                st.push(b * a);
            } else if item == "/" {
                st.push(b / a);
            }
        } else {
            let num = item.parse::<i32>().unwrap();
            st.push(num);
        }
    }

    st.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(8, calculate(vec!["5", "3", "+"]));
        assert_eq!(
            5,
            calculate(vec![
                "15", "7", "1", "1", "+", "-", "/", "3", "*", "2", "1", "1", "+", "+", "-"
            ])
        );
    }
}
