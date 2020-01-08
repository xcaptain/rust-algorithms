// https://leetcode-cn.com/problems/valid-sudoku/

use std::collections::HashMap;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: Vec<HashMap<usize, usize>> = vec![HashMap::new(); 9];
    let mut cols: Vec<HashMap<usize, usize>> = vec![HashMap::new(); 9];
    let mut boxes: Vec<HashMap<usize, usize>> = vec![HashMap::new(); 9];
    for (i, row) in board.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let num = *val;
            if num != '.' {
                let n = ((num as u8) - 48_u8) as usize;
                let box_idx = ((i / 3) * 3 + j / 3) as usize;

                let counter1 = rows[i].entry(n).or_insert(0);
                *counter1 += 1;

                let counter2 = cols[j].entry(n).or_insert(0);
                *counter2 += 1;

                let counter3 = boxes[box_idx].entry(n).or_insert(0);
                *counter3 += 1;

                if *counter1 > 1 || *counter2 > 1 || *counter3 > 1 {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p36() {
        let b1 = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(true, is_valid_sudoku(b1));
    }
}
