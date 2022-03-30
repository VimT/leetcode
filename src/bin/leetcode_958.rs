//! 二叉树的完全性检验

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut q = vec![root];
    let mut is_final = false;
    while !q.is_empty() {
        let mut nq = Vec::with_capacity(q.len() * 2);
        for root in q {
            let node = root.as_ref().unwrap().borrow();
            if node.left.is_some() {
                if is_final { return false; }
                nq.push(node.left.clone());
            } else {
                is_final = true;
            }
            if node.right.is_some() {
                if is_final { return false; }
                nq.push(node.right.clone());
            } else {
                is_final = true;
            }
        }
        q = nq;
    }
    true
}


fn main() {
    assert_eq!(is_complete_tree(tree![1,2,3,4,5,6,7,8,9,10,11,12,13,null,null,15]), false);
    assert_eq!(is_complete_tree(tree![1,2,3,4,5,6]), true);
    assert_eq!(is_complete_tree(tree![1,2,3,4,5,null,7]), false);
}
