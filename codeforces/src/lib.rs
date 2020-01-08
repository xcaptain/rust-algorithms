pub mod p112a;
pub mod p116a;
pub mod p118a;
pub mod p122a;
pub mod p131a;
pub mod p133a;
pub mod p158a;
pub mod p158b;
pub mod p160a;
pub mod p231a;
pub mod p236a;
pub mod p263a;
pub mod p266a;
pub mod p281a;
pub mod p282a;
pub mod p339a;
pub mod p41a;
pub mod p467a;
pub mod p50a;
pub mod p546a;
pub mod p58a;
pub mod p69a;
pub mod p71a;
pub mod p96a;

use std::io::{BufReader, BufWriter, Read, Write};

pub struct Scanner<U: Sized + Read> {
    pub buffer: Vec<String>,
    pub reader: U,
}
impl<U: Sized + Read> Scanner<U> {
    pub fn next_line<T: std::str::FromStr>(&mut self) -> T {
        // loop {
        //     if let Some(token) = self.buffer.pop() {
        //         return token.parse().ok().expect("Failed parse");
        //     }
        //     let mut input = String::new();
        //     self.reader.read_to_string(&mut input).expect("Failed read");
        //     self.buffer = input.lines().rev().map(String::from).collect();
        // }
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
