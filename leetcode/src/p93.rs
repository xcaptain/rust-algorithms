// https://leetcode-cn.com/problems/restore-ip-addresses/

pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let l = s.len();
    if l == 0 || l > 12 {
        return vec![];
    }
    let mut res = vec![];
    backtrack(&s, 0, s.len(), vec![], &mut res);

    // join to beautiful ip
    res.iter().map(|row| row.join(".")).collect::<Vec<String>>()
}

fn backtrack(ip: &String, start: usize, end: usize, cur: Vec<String>, res: &mut Vec<Vec<String>>) {
    if end == start {
        // println!("cur: {:?}", cur);
        if cur.len() == 4 {
            res.push(cur);
        }
        return;
    }
    if is_ip_seg(&ip[start..=start]) {
        let mut new_cur = cur.clone();
        new_cur.push(ip[start..=start].to_owned());
        backtrack(ip, start + 1, end, new_cur, res);
    }
    if start + 1 < end && is_ip_seg(&ip[start..=start + 1]) {
        let mut new_cur = cur.clone();
        new_cur.push(ip[start..=start + 1].to_owned());
        backtrack(ip, start + 2, end, new_cur, res);
    }
    if start + 2 < end && is_ip_seg(&ip[start..=start + 2]) {
        let mut new_cur = cur.clone();
        new_cur.push(ip[start..=start + 2].to_owned());
        backtrack(ip, start + 3, end, new_cur, res);
    }
}

fn is_ip_seg(seg: &str) -> bool {
    if seg.is_empty() || seg.len() > 3 {
        return false;
    }
    if &seg[0..=0] == "0" && seg.len() > 1 {
        return false;
    }
    let num = seg.parse::<usize>().unwrap();
    if num > 255 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p93() {
        assert_eq!(
            vec!["255.255.11.135", "255.255.111.35"],
            restore_ip_addresses(String::from("25525511135"))
        );
        assert_eq!(
            vec!["0.10.0.10", "0.100.1.0"],
            restore_ip_addresses(String::from("010010"))
        );
    }
}
