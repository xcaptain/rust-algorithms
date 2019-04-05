use regex::Regex;
use std::cmp::min;
use std::str::FromStr;

#[derive(Clone)]
pub enum NodeKind {
    Static, // pure string
    Regex,  // contains regex
}

#[derive(Clone)]
pub struct TreeNode {
    prefix: &'static str,
    edges: Vec<Box<TreeNode>>,
    is_word: bool,
    kind: NodeKind,
}

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            prefix: "",
            edges: vec![],
            is_word: false,
            kind: NodeKind::Static,
        }
    }

    pub fn insert(&mut self, word: &'static str) {
        if word.is_empty() {
            return;
        } else if &word[..1] == "{" {
            self.insert_start_with_regex(word);
        } else {
            self.insert_not_start_with_regex(word);
        }
    }

    fn insert_start_with_regex(&mut self, word: &'static str) {
        let right_braket_pos = word.find("}").unwrap();
        let regex_word = &word[1..right_braket_pos];
        let mut new_node = TreeNode {
            prefix: regex_word,
            edges: vec![],
            is_word: false,
            kind: NodeKind::Regex,
        };
        let word = &word[right_braket_pos + 1..];
        if word.is_empty() {
            new_node.is_word = true;
        } else {
            new_node.insert(word);
        }
        self.edges.push(Box::new(new_node));
    }

    fn insert_not_start_with_regex(&mut self, word: &'static str) {
        let mut word = word;
        for edge in self.edges.iter_mut() {
            let longest_prefix_len = common_prefix_len(&edge.prefix, &word);
            if longest_prefix_len == 0 {
                continue;
            }
            // 拆分新插入的字符串
            let old_node_prefix = &edge.prefix[longest_prefix_len..];
            let common_prefix = &edge.prefix[..longest_prefix_len];
            word = &word[longest_prefix_len..];
            if old_node_prefix.is_empty() {
                // 新的子节点插入到这个节点下
                edge.insert(word);
            } else {
                (*edge).prefix = old_node_prefix; // 更新原结点字符串
                let mut new_parent_node = TreeNode {
                    prefix: common_prefix,
                    edges: vec![edge.clone()],
                    is_word: false,
                    kind: NodeKind::Static,
                };
                if word.is_empty() {
                    new_parent_node.is_word = true;
                } else {
                    new_parent_node.insert(word);
                }
                *edge = Box::new(new_parent_node);
            }
        }

        let left_bracket = word.find("{");
        if let Some(left_bracket_pos) = left_bracket {
            if left_bracket_pos == 0 {
                // 不可能走这个分支，因为这个方法只处理第一个字符不为`{`的字符串
                self.insert_start_with_regex(word);
            } else {
                // 把纯字符串部分插入，然后把剩下的`{`的部分递归插入，因为目前节点为空所以直接push就行
                let static_word = &word[..left_bracket_pos];
                let mut static_node = TreeNode {
                    prefix: static_word,
                    edges: vec![],
                    is_word: false,
                    kind: NodeKind::Static,
                };
                word = &word[left_bracket_pos..];
                if word.is_empty() {
                    static_node.is_word = true;
                } else {
                    static_node.insert_start_with_regex(word);
                }
                self.edges.push(Box::new(static_node));
            }
        } else {
            let static_node = TreeNode {
                prefix: word,
                edges: vec![],
                is_word: true,
                kind: NodeKind::Static,
            };
            self.edges.push(Box::new(static_node));
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut search = word;
        for edge in &self.edges {
            match edge.kind {
                NodeKind::Static => {
                    match search.find(&edge.prefix) {
                        Some(0) => {
                            // 如果匹配上了当前节点则继续往下匹配
                            search = &search[edge.prefix.len()..];
                            if search.is_empty() {
                                return edge.is_word;
                            }
                            return edge.contains(search);
                        }
                        _ => {
                            // 否则跳过当前边匹配下一条
                            continue;
                        }
                    }
                }
                NodeKind::Regex => {
                    // match a word to a regex
                    let regex_pattern = edge.prefix;
                    let re = Regex::from_str(regex_pattern).unwrap();
                    let result = re.find(search);
                    match result {
                        Some(matcher) => {
                            let start_pos = matcher.start();
                            if start_pos == 0 {
                                // match at start
                                let end_pos = matcher.end();
                                search = &search[end_pos..];
                                if search.is_empty() {
                                    return edge.is_word;
                                }
                                return edge.contains(search);
                            }
                        }
                        None => {
                            return false;
                        }
                    }
                }
            }
        }
        return false;
    }
}

// return longest consequence substring of s1 and s2
pub fn common_prefix_len(s1: &str, s2: &str) -> usize {
    let l = min(s1.len(), s2.len());
    let mut start = 0;
    let s1_arr = s1.as_bytes();
    let s2_arr = s2.as_bytes();
    let left_bracket = "{".as_bytes();

    while start < l {
        if s1_arr[start] == s2_arr[start] {
            start += 1;
        } else if s1_arr[start] == left_bracket[0] || s2_arr[start] == left_bracket[0] {
            break;
        } else {
            break;
        }
    }
    return start;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut t = TreeNode::new();
        t.insert("apple");
        assert_eq!("apple", t.edges[0].prefix);

        t.insert("ape");
        assert_eq!("ap", t.edges[0].prefix);
        assert_eq!("ple", t.edges[0].edges[0].prefix);
        assert_eq!("e", t.edges[0].edges[1].prefix);

        t.insert("app");
        assert_eq!("p", t.edges[0].edges[0].prefix);
        assert_eq!("le", t.edges[0].edges[0].edges[0].prefix);
    }

    #[test]
    fn test_search() {
        let mut t = TreeNode::new();
        t.insert("apple");
        t.insert("ape");
        t.insert("app");
        t.insert("{[0-9]+}");
        t.insert("app{[0-9]+}");
        t.insert("/books/{[0-9]+}/comments/");

        assert!(t.contains("apple"));
        assert!(t.contains("ape"));
        assert!(t.contains("app"));
        assert_eq!(false, t.contains("apb"));
        assert_eq!(false, t.contains("ap"));
        assert!(t.contains("1234"));
        assert!(t.contains("app12345"));
        assert!(t.contains("/books/12/comments/"));
    }

    #[test]
    fn test_common_prefix_len() {
        assert_eq!(3, common_prefix_len("apple", "app"));
        assert_eq!(2, common_prefix_len("apple", "ape"));
        assert_eq!(2, common_prefix_len("ap{[1-9]+}", "ap123"));
    }
}
