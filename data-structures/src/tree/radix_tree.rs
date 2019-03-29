use std::cmp::min;

#[derive(Clone)]
pub struct TreeNode {
    prefix: String,
    edges: Vec<Box<TreeNode>>,
    is_word: bool,
}

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            prefix: "".to_string(),
            edges: vec![],
            is_word: false,
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut word = word;
        for edge in self.edges.iter_mut() {
            let longest_prefix = lcs(edge.prefix.clone(), word.clone());
            if longest_prefix.is_empty() {
                continue;
            }
            // 拆分新插入的字符串
            let longest_prefix_len = longest_prefix.len();
            // let new_parent_word = word[longest_prefix_len..].to_string();
            let old_node_prefix = edge.prefix[longest_prefix_len..].to_string();
            if old_node_prefix.is_empty() {
                // 新的子节点插入到这个节点下
                word = word[longest_prefix_len..].to_string();
                edge.insert(word.clone());
            } else {
                (*edge).prefix = old_node_prefix.clone(); // 更新原结点字符串
                let mut new_parent_node = TreeNode {
                    prefix: longest_prefix,
                    edges: vec![edge.clone()],
                    is_word: false,
                };
                word = word[longest_prefix_len..].to_string();
                if word.is_empty() {
                    new_parent_node.is_word = true;
                }
                new_parent_node.insert(word.clone());
                *edge = Box::new(new_parent_node);
            }
        }
        let new_node = TreeNode {
            prefix: word,
            edges: vec![],
            is_word: true,
        };
        self.edges.push(Box::new(new_node));
    }

    pub fn contains(&self, word: String) -> bool {
        let mut search = word.clone();
        for edge in &self.edges {
            match search.find(&edge.prefix) {
                Some(0) => {
                    // 如果匹配上了当前节点则继续往下匹配
                    search = search[edge.prefix.len()..].to_string();
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
        return false;
    }
}

// return longest consequence substring of s1 and s2
pub fn lcs(s1: String, s2: String) -> String {
    let l = min(s1.len(), s2.len());
    let mut start = 0;
    while start < l {
        if s1.as_str().as_bytes()[start] == s2.as_str().as_bytes()[start] {
            start += 1;
        } else {
            break;
        }
    }
    return s1[..start].to_string();
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut t = TreeNode::new();
        t.insert("apple".to_string());
        assert_eq!("apple".to_string(), t.edges[0].prefix);

        t.insert("ape".to_string());
        assert_eq!(&"ap".to_string(), &t.edges[0].prefix);
        assert_eq!(&"ple".to_string(), &t.edges[0].edges[0].prefix);
        assert_eq!(&"e".to_string(), &t.edges[0].edges[1].prefix);

        t.insert("app".to_string());
        assert_eq!(&"p".to_string(), &t.edges[0].edges[0].prefix);
        assert_eq!(&"le".to_string(), &t.edges[0].edges[0].edges[0].prefix);
    }

    #[test]
    fn test_search() {
        let mut t = TreeNode::new();
        t.insert("apple".to_string());
        t.insert("ape".to_string());
        t.insert("app".to_string());

        assert!(t.contains("apple".to_string()));
        assert!(t.contains("ape".to_string()));
        assert!(t.contains("app".to_string()));
        assert_eq!(false, t.contains("apb".to_string()));
        assert_eq!(false, t.contains("ap".to_string()));
    }

    #[test]
    fn test_lcs() {
        assert_eq!(
            String::from("app"),
            lcs(String::from("apple"), String::from("app"))
        );

        assert_eq!(
            String::from("ap"),
            lcs(String::from("apple"), String::from("ape"))
        );
    }
}
