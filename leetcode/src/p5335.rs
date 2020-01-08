// https://leetcode-cn.com/contest/weekly-contest-175/problems/maximum-students-taking-exam/
// TODO: 感觉可以按照八皇后的思路用回溯来解

pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    let row_len = seats.len();
    let col_len = seats[0].len();
    let mut total_available = vec![];
    for row_idx in 0..row_len {
        for col_idx in 0..col_len {
            if seats[row_idx][col_idx] == '.' {
                total_available.push([row_idx as i32, col_idx as i32]);
            }
        }
    }

    let mut init_pos = vec![total_available[0]];
    for item in total_available {
        let cur = max_helper(init_pos, item);
        if cur > res {
            res = cur;
        }
    }
    cur
}

fn max_helper(init_pos: Vec<[i32; 2]>, added: [i32; 2]) -> i32 {
    
}

fn is_conflict(cord1: [i32; 2], cord2: [i32; 2]) -> bool {
    if cord1[0] == cord2[0] { // same row
        if (cord1[1] - cord2[1]).abs() <= 1 {
            return true;
        }
    } else if (cord1[0] - cord2[0]).abs() == 1 {
        if (cord1[1] - cord2[1]).abs() == 1 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p5335() {
        assert_eq!(
            4,
            max_students(vec![
                vec!['#', '.', '#', '#', '.', '#'],
                vec!['.', '#', '#', '#', '#', '.'],
                vec!['#', '.', '#', '#', '.', '#'],
            ])
        );

        assert_eq!(
            10,
            max_students(vec![
                vec!['#', '.', '.', '.', '#'],
                vec!['.', '#', '.', '#', '.'],
                vec!['.', '.', '#', '.', '.'],
                vec!['.', '#', '.', '#', '.'],
                vec!['#', '.', '.', '.', '#']
            ])
        );
    }
}
