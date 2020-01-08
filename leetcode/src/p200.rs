// https://leetcode-cn.com/problems/number-of-islands/

use std::collections::VecDeque;

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let mut grid = grid;
    let mut res = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' {
                bfs(&mut grid, i as i32, j as i32);
                res += 1;
            }
        }
    }
    res
}

/// bfs 遍历，更新grid，返回岛屿面积
fn bfs(grid: &mut Vec<Vec<char>>, row: i32, col: i32) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut visited = VecDeque::new();
    visited.push_back([row, col]);

    let mut res = 0;
    while !visited.is_empty() {
        let [cur_row, cur_col] = visited.pop_front().unwrap();
        if cur_row < 0
            || cur_col < 0
            || cur_row == rows
            || cur_col == cols
            || grid[cur_row as usize][cur_col as usize] != '1'
        {
            continue;
        }
        res += 1;
        grid[cur_row as usize][cur_col as usize] = '0';
        if cur_row + 1 < rows && grid[(cur_row + 1) as usize][cur_col as usize] == '1' {
            visited.push_back([cur_row + 1, cur_col]);
        }
        if cur_row >= 1 && grid[(cur_row - 1) as usize][cur_col as usize] == '1' {
            visited.push_back([cur_row - 1, cur_col]);
        }
        if cur_col + 1 < cols && grid[cur_row as usize][(cur_col + 1) as usize] == '1' {
            visited.push_back([cur_row, cur_col + 1]);
        }
        if cur_col >= 1 && grid[cur_row as usize][(cur_col - 1) as usize] == '1' {
            visited.push_back([cur_row, cur_col - 1]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p200() {
        assert_eq!(
            1,
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ])
        );

        assert_eq!(0, num_islands(vec![]));
    }
}
