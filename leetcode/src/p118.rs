// https://leetcode-cn.com/problems/pascals-triangle/

// TODO: recursive is not good, should come up with a
// better iteration solution

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    generate_rec(num_rows)
}

fn generate_rec(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 0 {
        vec![]
    } else if num_rows == 1 {
        vec![vec![1]]
    } else if num_rows == 2 {
        vec![vec![1], vec![1, 1]]
    } else {
        let mut previous = generate_rec(num_rows - 1);
        let previous_last = previous.last().unwrap();
        let mut next_row = vec![1];
        for i in 0..previous_last.len() - 1 {
            next_row.push(previous_last[i] + previous_last[i + 1]);
        }
        next_row.push(1);
        previous.push(next_row);
        previous
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let t5 = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(t5, generate(5));
    }
}
