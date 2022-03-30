//! 二叉树的前序遍历

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() { return vec![]; }
    let mut s = VecDeque::new();
    s.push_back(root);
    let mut ans = vec![];
    while !s.is_empty() {
        let n = s.pop_back().unwrap();
        let node = n.as_ref().unwrap().borrow();
        ans.push(node.val);
        if node.right.is_some() {
            s.push_back(node.right.clone());
        }
        if node.left.is_some() {
            s.push_back(node.left.clone());
        }
    }

    ans
}

fn main() {
    assert_eq!(preorder_traversal(tree![1,null,2,3]), [1, 2, 3]);
    assert_eq!(preorder_traversal(tree![]), []);
    assert_eq!(preorder_traversal(tree![1]), [1]);
}
