use std::collections::HashMap;

pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
    let mut ratios: Vec<Decimal> = vec![Decimal::new(0, 1); rectangles.len()];
    for (idx, rec) in rectangles.iter().enumerate() {
        ratios[idx] = Decimal::new(rec[0], rec[1]);
    }

    let mut m = HashMap::new();
    for ratio in ratios {
        let counter = m.entry(ratio).or_insert(0);
        *counter += 1;
    }
    let mut ret = 0;
    for (_, count) in m {
        if count >= 2 {
            ret += count * (count - 1) / 2;
        }
    }
    ret
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Decimal {
    pub up: i32,
    pub down: i32,
}

impl Decimal {
    pub fn new(up: i32, down: i32) -> Self {
        let gcd = gcd1(up, down);
        return Decimal {
            up: up / gcd,
            down: down / gcd,
        };
    }
}

pub fn gcd1(m: i32, n: i32) -> i32 {
    let g = m % n;
    if g == 0 {
        return n;
    }
    gcd1(n, g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interchangeable_rectangles() {
        #[derive(Debug, Clone)]
        struct Args {
            rectangles: Vec<Vec<i32>>,
        }
        #[derive(Debug, Clone)]
        struct Test {
            name: String,
            args: Args,
            want: i64,
        }
        let tests = vec![
            Test {
                name: String::from("case1"),
                args: Args {
                    rectangles: vec![vec![4, 8], vec![3, 6], vec![10, 20], vec![15, 30]],
                },
                want: 6,
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    rectangles: vec![vec![4, 5], vec![7, 8]],
                },
                want: 0,
            },
        ];

        for test in tests {
            assert_eq!(
                test.want,
                interchangeable_rectangles(test.args.rectangles),
                "{} fails",
                test.name
            );
        }
    }
}
