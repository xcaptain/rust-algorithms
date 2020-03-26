//! This problem was asked by Pinterest.
//! At a party, there is a single person who everyone knows, but who does not know anyone in return (the "celebrity").
//! To help figure out who this is, you have access to an O(1) method called knows(a, b), which returns True
//! if person a knows person b, else False.
//! Given a list of N people and the above operation, find a way to identify the celebrity in O(N) time.
//!
//! Hint, given a relation table, we need to find a number such the row is all 0 and the column is all 1
//!
//! [0, 0, 1, 0],
//! [0, 0, 1, 0],
//! [0, 0, 0, 0],
//! [0, 0, 1, 0],
//!
//! In this example, the index is 2

pub fn find_celebrity(tbl: Vec<Vec<i32>>) -> Option<usize> {
    let total_people_num = tbl.len();
    let mut st: Vec<usize> = (0..total_people_num).collect();
    let mut a = 0;
    let mut b = 0;

    while st.len() > 1 {
        a = st.pop().unwrap();
        b = st.pop().unwrap();

        if tbl[a][b] == 1 {
            // A knows B, A can't be celebrity
            a = st.pop().unwrap();
        } else {
            // A doesn't know B, B can't be celebrity, because every celebrity
            // should be known by others
            b = st.pop().unwrap();
        }
    }
    let mut c = st.pop().unwrap();
    if tbl[c][b] == 1 {
        // C knows B, C can't be celebrity, B may be
        c = b;
    }
    if tbl[c][a] == 1 {
        // C knows 1, C can't be celebrity, A may be
        c = a;
    }

    // C may be celebrity now, but need to compare with all others again
    for i in 0..total_people_num {
        if i != c && (tbl[c][i] == 1 || tbl[i][c] == 0) {
            return None;
        }
    }

    Some(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_celebrity() {
        let tbl1 = vec![
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 1, 0],
        ];
        assert_eq!(Some(2), find_celebrity(tbl1));
    }
}
