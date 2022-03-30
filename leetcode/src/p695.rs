// https://leetcode-cn.com/problems/max-area-of-island/

use std::collections::VecDeque;

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut res = 0;
    let mut grid = grid;
    for row in 0..rows {
        for col in 0..cols {
            // 针对cur进行dfs，获取最大连续1的个数
            // let num = dfs(&mut grid, row as i32, col as i32);
            if grid[row][col] == 1 {
                let num = bfs(&mut grid, row as i32, col as i32);
                res = res.max(num);
            }
        }
    }
    res
}

/// 使用dfs计算最大连续的1的个数
#[allow(dead_code)]
fn dfs(grid: &mut Vec<Vec<i32>>, row: i32, col: i32) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    if row == rows || row < 0 || col == cols || col < 0 {
        return 0;
    }
    if grid[row as usize][col as usize] == 1 {
        grid[row as usize][col as usize] = 0;
        return 1
            + dfs(grid, row + 1, col)
            + dfs(grid, row - 1, col)
            + dfs(grid, row, col + 1)
            + dfs(grid, row, col - 1);
    }
    0
}

/// bfs using iteration, the common way is to add a `used` array
/// to hold the used points, but this way is too slow, just set
/// every visited point to zero is much simpler
fn bfs(grid: &mut Vec<Vec<i32>>, row: i32, col: i32) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut visited: VecDeque<[i32; 2]> = VecDeque::new();
    visited.push_back([row, col]);

    // let mut used = vec![];
    let mut res = 0;
    while !visited.is_empty() {
        let [cur_row, cur_col] = visited.pop_front().unwrap();
        // if !used.contains(&[cur_row, cur_col]) {
        //     used.push([cur_row, cur_col]);
        //     grid[cur_row][cur_col] = 0;
        // }
        if cur_row < 0
            || cur_col < 0
            || cur_row == rows
            || cur_col == cols
            || grid[cur_row as usize][cur_col as usize] != 1
        {
            continue;
        }
        res += 1;
        grid[cur_row as usize][cur_col as usize] = 0;
        if cur_row + 1 < rows && grid[(cur_row + 1) as usize][cur_col as usize] == 1 {
            visited.push_back([cur_row + 1, cur_col]);
        }
        if cur_row >= 1 && grid[(cur_row - 1) as usize][cur_col as usize] == 1 {
            visited.push_back([cur_row - 1, cur_col]);
        }
        if cur_col + 1 < cols && grid[cur_row as usize][(cur_col + 1) as usize] == 1 {
            visited.push_back([cur_row, cur_col + 1]);
        }
        if cur_col >= 1 && grid[cur_row as usize][(cur_col - 1) as usize] == 1 {
            visited.push_back([cur_row, cur_col - 1]);
        }
    }
    // println!("used: {:?}", used);

    // return used.len() as i32;
    res
}

// TODO: use bfs to search the graph

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p695() {
        assert_eq!(
            6,
            max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            ])
        );

        assert_eq!(
            4,
            max_area_of_island(vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 1, 1]
            ])
        );
    }
}
