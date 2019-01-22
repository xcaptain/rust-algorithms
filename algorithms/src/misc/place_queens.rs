pub fn place_queens(Q: Vec<usize>, r: usize) {
    if r == Q.len() - 1 {
        println!("{:?}", Q);
    }
    for j in 0..Q.len() {
        let mut legal = true;
        for i in 0..r-1 {
            if Q[i] == j || Q[i] == j + r - i || Q[i] == j - r + i {
                legal = false;
            }
        }
        if legal {
            Q[r] = j;
            place_queens(Q, r+1);
        }
    }
}
