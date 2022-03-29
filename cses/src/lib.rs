pub mod p1068;

use std::io::{BufReader, BufWriter, Read, Write};

pub struct Scanner<U: Sized + Read> {
    pub buffer: Vec<String>,
    pub reader: U,
}
impl<U: Sized + Read> Scanner<U> {
    pub fn next_line<T: std::str::FromStr>(&mut self) -> T {
        if self.buffer.is_empty() {
            let mut input = String::new();
            self.reader.read_to_string(&mut input).expect("Failed read");
            self.buffer = input.lines().rev().map(String::from).collect();
        }
        self.buffer
            .pop()
            .unwrap()
            .parse()
            .ok()
            .expect("Failed parse")
    }

    pub fn new(reader: U) -> Self {
        Scanner {
            buffer: vec![],
            reader,
        }
    }
}

type Solution = fn(&mut dyn Read, &mut dyn Write);

/// cases is the test cases of the problem, the first item is input, second
/// is output, just copy from the website
pub fn test_helper(cases: Vec<[&str; 2]>, solution: Solution) {
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
