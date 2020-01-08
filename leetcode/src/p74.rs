// https://leetcode-cn.com/problems/search-a-2d-matrix/

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    if rows == 0 {
        return false;
    }
    let cols = matrix[0].len();
    if cols == 0 {
        return false;
    }

    for i in 0..rows {
        let row: &Vec<i32> = &matrix[i];
        if target >= row[0] && target <= row[cols - 1] {
            // binary search here is faster then contains
            return match row.binary_search(&target) {
                Ok(_idx) => true,
                Err(_) => false,
            };
        // return row.contains(&target);
        } else if target < row[0] {
            return false;
        } else if target > row[cols - 1] {
            continue;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p74() {
        assert_eq!(
            true,
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                3
            )
        );
    }
}
