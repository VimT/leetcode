//! 单值二叉树

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let val = root.as_ref().unwrap().borrow().val;
    let mut q = VecDeque::new();
    q.push_back(root);
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        let n = node.as_ref().unwrap().borrow();
        if n.val != val { return false; }
        if n.left.is_some() { q.push_back(n.left.clone()); }
        if n.right.is_some() { q.push_back(n.right.clone()); }
    }
    true
}


fn main() {
    assert_eq!(is_unival_tree(tree![1,1,1,1,1,null,1]), true);
    assert_eq!(is_unival_tree(tree![2,2,2,5,2]), false);
}
