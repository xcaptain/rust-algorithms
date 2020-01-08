// https://leetcode-cn.com/problems/n-queens/

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    if n == 1 {
        return vec![vec![String::from("Q")]];
    } else if n < 4 {
        // let s = ".".repeat(n as usize);
        // return vec![vec![s; 3]; 1];
        return vec![];
    }
    let methods = nqueen(n as usize);
    let cols = methods[0].len();
    let mut res = vec![];
    for method in methods {
        let mut t = vec![];
        for val in method {
            // res[row][*val as usize] = String::from("Q");
            let mut arr = vec!['.'; cols];
            arr[val as usize] = 'Q';
            let s = arr
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("");
            t.push(s);
        }
        res.push(t);
    }
    res
}

pub fn nqueen(n: usize) -> Vec<Vec<u128>> {
    let mut board = vec![0; n];
    let mut result = vec![];
    backtrack(&mut board, 0, &mut result);
    result
}

/// algorithms from: http://jeffe.cs.illinois.edu/teaching/algorithms/book/02-backtracking.pdf
/// board: contains an available placement for current board
/// r: the current placement row, starting from 0 to n-1
/// result: the available placement array
fn backtrack(board: &mut Vec<u128>, r: usize, result: &mut Vec<Vec<u128>>) {
    let n = board.len();
    if r == n {
        result.push(board.clone());
    } else {
        for j in 0..n {
            let mut legal = true;
            for (i, boardi) in board.iter().enumerate().take(r) {
                if *boardi == (j as u128)
                    || (j + r >= i && *boardi == ((j + r - i) as u128))
                    || (j + i >= r && *boardi == ((j + i - r) as u128))
                {
                    legal = false;
                }
            }
            if legal {
                board[r] = j as u128;
                backtrack(board, r + 1, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p51() {
        assert_eq!(vec![vec!["Q"],], solve_n_queens(1));

        let exp: Vec<Vec<String>> = vec![];
        assert_eq!(exp, solve_n_queens(3));

        // [[2, 4, 1, 3], [3, 1, 4, 2]]
        assert_eq!(
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."],
            ],
            solve_n_queens(4)
        );
    }
}
