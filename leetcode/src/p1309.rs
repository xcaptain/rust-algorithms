// https://leetcode-cn.com/problems/decrypt-string-from-alphabet-to-integer-mapping/

use std::collections::HashMap;

pub fn freq_alphabets(s: String) -> String {
    // should find a simpler way to zip an array to hashmap
    let v1 = vec![
        ("1", 'a'),
        ("2", 'b'),
        ("3", 'c'),
        ("4", 'd'),
        ("5", 'e'),
        ("6", 'f'),
        ("7", 'g'),
        ("8", 'h'),
        ("9", 'i'),
    ];
    let mut m1: HashMap<&str, String> = HashMap::new();
    for row in v1 {
        m1.insert(row.0, row.1.to_string());
    }
    let v2 = vec![
        ("10#", 'j'),
        ("11#", 'k'),
        ("12#", 'l'),
        ("13#", 'm'),
        ("14#", 'n'),
        ("15#", 'o'),
        ("16#", 'p'),
        ("17#", 'q'),
        ("18#", 'r'),
        ("19#", 's'),
        ("20#", 't'),
        ("21#", 'u'),
        ("22#", 'v'),
        ("23#", 'w'),
        ("24#", 'x'),
        ("25#", 'y'),
        ("26#", 'z'),
    ];
    let mut m2: HashMap<&str, String> = HashMap::new();
    for row in v2 {
        m2.insert(row.0, row.1.to_string());
    }
    let mut i = 0;
    let len = s.len();
    let arr: Vec<char> = s.chars().collect();
    let mut ans: Vec<String> = vec![];
    while i < len {
        if i + 2 < len && arr[i + 1] != '#' && arr[i + 2] == '#' {
            let mv2 = m2.get(&s[i..i + 3]).unwrap().to_owned();
            ans.push(mv2);
            i += 3;
        } else {
            let mv1 = m1.get(&s[i..=i]).unwrap().to_owned();
            ans.push(mv1);
            i += 1;
        }
    }
    ans.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1309() {
        assert_eq!(
            String::from("abcdefghijklmnopqrstuvwxyz"),
            freq_alphabets(String::from(
                "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#"
            ))
        );
    }
}
