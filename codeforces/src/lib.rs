pub mod p71a;

use std::io::Read;

#[derive(Default)]
pub struct Scanner<U: Sized + Read> {
    pub buffer: Vec<String>,
    pub reader: U,
}
impl<U: Sized + Read> Scanner<U> {
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_to_string(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn new(reader: U) -> Self {
        return Scanner {
            buffer: vec![],
            reader,
        };
    }
}
