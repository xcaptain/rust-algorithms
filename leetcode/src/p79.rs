// https://leetcode-cn.com/problems/word-search/

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let rows = board.len();
    let cols = board[0].len();

    let l = word.len();
    let word_arr: Vec<char> = word.chars().collect();
    let mut marked = vec![vec![false; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            if backtrack(&board, &word_arr, &mut marked, 0, l - 1, i as i32, j as i32) {
                return true;
            }
        }
    }

    false
}

fn backtrack(
    board: &Vec<Vec<char>>,
    word_arr: &Vec<char>,
    marked: &mut Vec<Vec<bool>>,
    start: usize,
    end: usize,
    i: i32,
    j: i32,
) -> bool {
    let rows = board.len();
    let cols = board[0].len();

    if start == end {
        return board[i as usize][j as usize] == word_arr[start];
    }
    let directions = vec![[-1, 0], [1, 0], [0, -1], [0, 1]];
    if board[i as usize][j as usize] == word_arr[start] {
        marked[i as usize][j as usize] = true;

        for direct in directions {
            let cur_i = i + direct[0];
            let cur_j = j + direct[1];

            if cur_i >= 0
                && cur_i < rows as i32
                && cur_j >= 0
                && cur_j < cols as i32
                && marked[cur_i as usize][cur_j as usize] == false
            {
                if backtrack(board, word_arr, marked, start + 1, end, cur_i, cur_j) {
                    return true;
                }
            }
        }
        marked[i as usize][j as usize] = false;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p79() {
        let matrix1 = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(true, exist(matrix1.clone(), String::from("ABCCED")));
        assert_eq!(true, exist(matrix1.clone(), String::from("SEE")));
        assert_eq!(false, exist(matrix1.clone(), String::from("ABCB")));
    }
}
