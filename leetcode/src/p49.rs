// https://leetcode-cn.com/problems/group-anagrams/

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let mut ss: Vec<char> = s.chars().collect();
        ss.sort();
        let skey = ss
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");
        let val = map.entry(skey).or_insert(vec![]);
        val.push(s);
    }

    let mut res = vec![];
    for (_k, v) in map {
        // println!("k: {}, v: {:?}", k, v);
        res.push(v);
    }
    res.sort();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p49() {
        assert_eq!(
            vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]],
            group_anagrams(vec![
                String::from("eat"),
                String::from("tea"),
                String::from("tan"),
                String::from("ate"),
                String::from("nat"),
                String::from("bat")
            ])
        );
    }
}
