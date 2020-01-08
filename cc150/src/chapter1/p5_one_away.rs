/// 判断1个字符串是否仅用0或1次变换就能得到另一个字符串，
/// 变换包括，插入，删除，替换一个字符
pub fn one_away(s1: String, s2: String) -> bool {
    let l1 = s1.len();
    let l2 = s2.len();
    if l1 == l2 {
        return one_edit_edit(s1, s2);
    } else if l1 >= l2 && l1 - l2 == 1 {
        return one_edit_insert(s2, s1);
    } else if l2 >= l1 && l2 - l1 == 1 {
        return one_edit_insert(s1, s2);
    }
    false
    // if (l1 > l2 && l1 - l2 >= 2) || (l1 < l2 && l2 - l1 >= 2) {
    //     return false;
    // }
    // true
}

/// test if s1 can become s2 in 1 edit op,
/// s1 and s2 have the same length
fn one_edit_edit(s1: String, s2: String) -> bool {
    let mut edited = false;
    for i in 0..s1.len() {
        if s1[i..=i] != s2[i..=i] {
            if edited == true {
                return false;
            }
            edited = true;
        }
    }
    true
}

/// test if s1 can become s2 using 1 insert op,
/// s1 is shorter and s2 is longer and s1.len() + 1 == s2.len()
fn one_edit_insert(s1: String, s2: String) -> bool {
    let mut edited = false;
    let mut i = 0;
    let mut j = 0;
    while i < s1.len() && j < s2.len() {
        if s1[i..=i] != s2[j..=j] {
            if edited == true {
                return false;
            }
            j += 1;
            edited = true;
        } else {
            i += 1;
            j += 1;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_away() {
        assert_eq!(true, one_away(String::from("pale"), String::from("ple")));
        assert_eq!(true, one_away(String::from("pales"), String::from("pale")));
        assert_eq!(true, one_away(String::from("pale"), String::from("bale")));
        assert_eq!(false, one_away(String::from("pal"), String::from("psa")));
    }
}
