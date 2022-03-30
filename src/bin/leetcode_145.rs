//! 二叉树的后序遍历

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() { return vec![]; }
    let mut s = VecDeque::new();
    s.push_back(root);
    let mut ans = vec![];
    while !s.is_empty() {
        let n = s.pop_back().unwrap();
        let node = n.as_ref().unwrap().borrow();
        ans.push(node.val);
        if node.left.is_some() { s.push_back(node.left.clone()); }
        if node.right.is_some() { s.push_back(node.right.clone()); }
    }
    ans.reverse();
    ans
}


fn main() {
    assert_eq!(postorder_traversal(tree![1,null,2,3]), [3, 2, 1]);
    assert_eq!(postorder_traversal(tree![]), []);
    assert_eq!(postorder_traversal(tree![1]), [1]);
}
