use data_structures::tree::binary_tree::Tree;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn find_cousins(t: Tree<i32>, val: i32) -> Vec<i32> {
    let res = bfs_traverse(t);

    // println!("{:?}", res);
    let nodes: Vec<(usize, Option<i32>, i32)> =
        res.iter().cloned().filter(|v| v.2 == val).collect();
    let depth = nodes[0].0;
    let parent_val = nodes[0].1;
    let ans: Vec<i32> = res
        .iter()
        .filter(|v| v.0 == depth && v.1 != parent_val)
        .map(|v| v.2)
        .collect();
    ans
}

/// depth, parent_val, val
fn bfs_traverse(t: Tree<i32>) -> Vec<(usize, Option<i32>, i32)> {
    let mut res = vec![];

    let mut q = VecDeque::new();
    if let Some(root_node) = t.root.as_ref() {
        q.push_back((Rc::clone(root_node), None));
    }

    let mut depth = 0;
    while !q.is_empty() {
        let l = q.len();
        for _i in 0..l {
            let (cur_node, parent_val) = q.pop_front().unwrap();
            let cur_val = cur_node.borrow().elem;
            res.push((depth, parent_val, cur_val));

            // if the current node has children, then push these children into
            // the queue, so we can continue traverse down the tree.
            // note: using map here is simpler than using match
            let borrowed_cur_node = cur_node.borrow();
            if let Some(v) = borrowed_cur_node.left.as_ref() {
                q.push_back((Rc::clone(v), Some(cur_val)));
            }
            if let Some(v) = borrowed_cur_node.right.as_ref() {
                q.push_back((Rc::clone(v), Some(cur_val)));
            }
        }

        depth += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cousins() {
        let tree1 = Tree::from_vec(vec![4, 2, 1, 3, 5, 6]);
        assert_eq!(vec![6], find_cousins(tree1, 1));
    }
}

// bZ68PTxdQ4p6
