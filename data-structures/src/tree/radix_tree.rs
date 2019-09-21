use regex::Regex;
use std::cmp::min;
use std::str::FromStr;

#[derive(Clone)]
pub enum NodeKind {
    Static, // pure string
    Regex,  // string contains regular expressions like: abc{[a-z]+}def
}

impl Default for NodeKind {
    fn default() -> Self {
        NodeKind::Static
    }
}

#[derive(Default, Clone)]
pub struct TreeNode {
    prefix: &'static str,
    edges: Vec<TreeNode>,
    is_word: bool,
    kind: NodeKind,
}

impl TreeNode {
    pub fn new(prefix: &'static str, kind: NodeKind) -> Self {
        TreeNode {
            prefix,
            edges: vec![],
            is_word: false,
            kind,
        }
    }

    /// insert a string into the radix tree
    pub fn insert(&mut self, word: &'static str) {
        if &word[..1] == "{" {
            self.insert_start_with_regex(word);
        } else {
            self.insert_not_start_with_regex(word);
        }
    }

    fn insert_start_with_regex(&mut self, word: &'static str) {
        assert!(word.chars().nth(0).unwrap() == '{'); // ensure word starts with `{`
        let right_braket_pos = word.find('}').unwrap();
        let regex_word = &word[1..right_braket_pos];
        let mut new_node = TreeNode::new(regex_word, NodeKind::Regex);
        let word = &word[right_braket_pos + 1..];
        if word.is_empty() {
            new_node.is_word = true;
        } else {
            new_node.insert(word);
        }
        self.edges.push(new_node);
    }

    fn insert_not_start_with_regex(&mut self, word: &'static str) {
        let mut word = word;
        for edge in self.edges.iter_mut() {
            let longest_prefix_len = common_prefix_len(&edge.prefix, &word);
            if longest_prefix_len == 0 {
                continue;
            }
            // split the prefix
            let old_node_prefix = &edge.prefix[longest_prefix_len..];
            let common_prefix = &edge.prefix[..longest_prefix_len];
            word = &word[longest_prefix_len..];
            // old node prefix is totally matched, like app matched by apple
            // keep `app` and insert `le` into `app`
            if old_node_prefix.is_empty() {
                edge.insert(word);
            } else {
                // the original node must be split into 2, like
                // `app` and `ape` becomes `ap` `p` and `e`
                (*edge).prefix = old_node_prefix;
                let mut new_parent_node = TreeNode::new(common_prefix, NodeKind::Static);
                new_parent_node.edges.push(edge.clone());
                if word.is_empty() {
                    new_parent_node.is_word = true;
                } else {
                    new_parent_node.insert(word);
                }
                *edge = new_parent_node;
            }
        }

        let left_bracket = word.find('{');
        if let Some(left_bracket_pos) = left_bracket {
            if left_bracket_pos == 0 {
                self.insert_start_with_regex(word);
            } else {
                // first create a node with pure string and then recursively insert the rest part
                let static_word = &word[..left_bracket_pos];
                let mut static_node = TreeNode::new(static_word, NodeKind::Static);
                word = &word[left_bracket_pos..];
                if word.is_empty() {
                    static_node.is_word = true;
                } else {
                    static_node.insert_start_with_regex(word);
                }
                self.edges.push(static_node);
            }
        } else {
            let mut static_node = TreeNode::new(word, NodeKind::Static);
            static_node.is_word = true;
            self.edges.push(static_node);
        }
    }

    /// check if a word is in the radix tree
    pub fn contains(&self, word: &str) -> bool {
        let mut search = word;
        for edge in &self.edges {
            match edge.kind {
                NodeKind::Static => {
                    match search.find(&edge.prefix) {
                        Some(0) => {
                            // current node is matched, recursively go down the tree
                            search = &search[edge.prefix.len()..];
                            if search.is_empty() {
                                return edge.is_word;
                            }
                            return edge.contains(search);
                        }
                        _ => {
                            // current node is not matched, skip to next node
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
        false
    }
}

#[derive(Default)]
pub struct Set {
    root: Box<TreeNode>,
}

impl Set {
    pub fn new() -> Self {
        Set {
            root: Box::new(TreeNode::new("", NodeKind::Static)),
        }
    }

    pub fn insert(&mut self, word: &'static str) {
        self.root.insert(word);
    }

    pub fn contains(&self, word: &'static str) -> bool {
        self.root.contains(word)
    }
}

#[derive(Default, Clone)]
struct TreeHashNode<T: Clone> {
    prefix: &'static str,
    edges: Vec<TreeHashNode<T>>,
    is_word: bool,
    kind: NodeKind,
    value: Option<T>,
}

impl<T: Clone> TreeHashNode<T> {
    pub fn new(prefix: &'static str, kind: NodeKind) -> Self {
        TreeHashNode {
            prefix,
            edges: vec![],
            is_word: false,
            kind,
            value: None,
        }
    }

    pub fn insert(&mut self, k: &'static str, v: T) {
        if &k[..1] == "{" {
            self.insert_start_with_regex(k, v);
        } else {
            self.insert_not_start_with_regex(k, v);
        }
    }

    fn insert_start_with_regex(&mut self, word: &'static str, v: T) {
        assert!(word.chars().nth(0).unwrap() == '{'); // ensure word starts with `{`
        let right_braket_pos = word.find('}').unwrap();
        let regex_word = &word[1..right_braket_pos];
        let mut new_node = TreeHashNode::new(regex_word, NodeKind::Regex);
        let word = &word[right_braket_pos + 1..];
        if word.is_empty() {
            new_node.is_word = true;
            new_node.value = Some(v);
        } else {
            new_node.insert(word, v);
        }
        self.edges.push(new_node);
    }

    fn insert_not_start_with_regex(&mut self, word: &'static str, v: T) {
        let mut word = word;
        for edge in self.edges.iter_mut() {
            let longest_prefix_len = common_prefix_len(&edge.prefix, &word);
            if longest_prefix_len == 0 {
                continue;
            }
            // split the prefix
            let old_node_prefix = &edge.prefix[longest_prefix_len..];
            let common_prefix = &edge.prefix[..longest_prefix_len];
            word = &word[longest_prefix_len..];
            // old node prefix is totally matched, like app matched by apple
            // keep `app` and insert `le` into `app`
            if old_node_prefix.is_empty() {
                edge.insert(word, v.clone());
            } else {
                // the original node must be split into 2, like
                // `app` and `ape` becomes `ap` `p` and `e`
                (*edge).prefix = old_node_prefix;
                let mut new_parent_node = TreeHashNode::new(common_prefix, NodeKind::Static);
                new_parent_node.edges.push(edge.clone());
                if word.is_empty() {
                    new_parent_node.is_word = true;
                    new_parent_node.value = Some(v.clone());
                } else {
                    new_parent_node.insert(word, v.clone());
                }
                *edge = new_parent_node;
            }
        }

        let left_bracket = word.find('{');
        if let Some(left_bracket_pos) = left_bracket {
            if left_bracket_pos == 0 {
                self.insert_start_with_regex(word, v);
            } else {
                // first create a node with pure string and then recursively insert the rest part
                let static_word = &word[..left_bracket_pos];
                let mut static_node = TreeHashNode::new(static_word, NodeKind::Static);
                word = &word[left_bracket_pos..];
                if word.is_empty() {
                    static_node.is_word = true;
                    static_node.value = Some(v);
                } else {
                    static_node.insert_start_with_regex(word, v);
                }
                self.edges.push(static_node);
            }
        } else {
            let mut static_node = TreeHashNode::new(word, NodeKind::Static);
            static_node.is_word = true;
            static_node.value = Some(v);
            self.edges.push(static_node);
        }
    }

    pub fn get(&self, word: &'static str) -> Option<&T> {
        let mut search = word;
        for edge in &self.edges {
            match edge.kind {
                NodeKind::Static => {
                    match search.find(&edge.prefix) {
                        Some(0) => {
                            // current node is matched, recursively go down the tree
                            search = &search[edge.prefix.len()..];
                            if search.is_empty() {
                                return edge.value.as_ref();
                            }
                            return edge.get(search);
                        }
                        _ => {
                            // current node is not matched, skip to next node
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
                                    return edge.value.as_ref();
                                }
                                return edge.get(search);
                            }
                        }
                        None => {
                            return None;
                        }
                    }
                }
            }
        }
        None
    }
}

#[derive(Default)]
pub struct Map<T: Clone> {
    root: Box<TreeHashNode<T>>,
}

impl<T: Clone> Map<T> {
    pub fn new() -> Self {
        Map {
            root: Box::new(TreeHashNode::new("", NodeKind::Static)),
        }
    }

    pub fn insert(&mut self, word: &'static str, v: T) {
        self.root.insert(word, v);
    }

    pub fn get(&self, word: &'static str) -> Option<&T> {
        self.root.get(word)
    }
}

// return longest consequence substring of s1 and s2
fn common_prefix_len(s1: &str, s2: &str) -> usize {
    let l = min(s1.len(), s2.len());
    let mut start = 0;
    let s1_arr = s1.as_bytes();
    let s2_arr = s2.as_bytes();

    while start < l {
        if s1_arr[start] == s2_arr[start] {
            start += 1;
        } else {
            break;
        }
    }
    start
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let mut t = Set::new();
        t.insert("apple");
        t.insert("ape");
        t.insert("app");
        t.insert("{[0-9]+}");
        t.insert("app{[0-9]+}");
        t.insert("/books/{[0-9]+}/comments/");
        t.insert("/books/book1/");

        assert!(t.contains("apple"));
        assert!(t.contains("ape"));
        assert!(t.contains("app"));
        assert_eq!(false, t.contains("apb"));
        assert_eq!(false, t.contains("ap"));
        assert!(t.contains("1234"));
        assert!(t.contains("app12345"));
        assert!(t.contains("/books/12/comments/"));
        assert!(t.contains("/books/book1/"));
    }

    #[test]
    fn test_common_prefix_len() {
        assert_eq!(3, common_prefix_len("apple", "app"));
        assert_eq!(2, common_prefix_len("apple", "ape"));
        assert_eq!(2, common_prefix_len("ap{[1-9]+}", "ap123"));
    }

    #[test]
    fn test_hash_search() {
        let mut m = Map::new();
        m.insert("/books/", 1);
        m.insert("/books/{[0-9]+}/", 2);
        m.insert("/books/book1/", 3);
        // TODO: add a match priority, static string should match first
        // m.insert("/books/123/", 4);

        assert_eq!(Some(&1), m.get("/books/"));
        assert_eq!(Some(&2), m.get("/books/1234/"));
        assert_eq!(Some(&3), m.get("/books/book1/"));
        // assert_eq!(Some(&4), m.get("/books/123/"));
    }
}
