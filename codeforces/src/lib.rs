pub mod p118a;
pub mod p158a;
pub mod p231a;
pub mod p71a;

use std::io::{BufReader, BufWriter, Read, Write};

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
            self.buffer = input.lines().rev().map(String::from).collect();
        }
    }

    pub fn new(reader: U) -> Self {
        return Scanner {
            buffer: vec![],
            reader,
        };
    }
}

type Solution = fn(&mut Read, &mut Write);
pub fn test_helper(cases: Vec<Vec<&str>>, solution: Solution) {
    for case in &cases {
        let mut input_file = BufReader::new(case[0].as_bytes());
        let mut out_file = BufWriter::new(Vec::new());
        solution(&mut input_file, &mut out_file);
        assert_eq!(
            case[1],
            String::from_utf8(out_file.into_inner().unwrap())
                .unwrap()
                .as_str()
                .trim()
        );
    }
}
