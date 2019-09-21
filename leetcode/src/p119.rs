// https://leetcode-cn.com/problems/pascals-triangle-ii/

pub fn get_row(row_index: i32) -> Vec<i32> {
    get_row_rec(row_index)
}

fn get_row_rec(row_index: i32) -> Vec<i32> {
    if row_index == 0 {
        return vec![1];
    } else if row_index == 1 {
        return vec![1, 1];
    } else {
        let previous_row = get_row_rec(row_index - 1);
        let mut next_row = vec![1];
        for i in 0..previous_row.len() - 1 {
            next_row.push(previous_row[i] + previous_row[i + 1]);
        }
        next_row.push(1);
        next_row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        assert_eq!(vec![1, 3, 3, 1], get_row(3));
    }
}
