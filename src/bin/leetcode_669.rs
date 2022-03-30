//! 修剪二叉搜索树


use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn trim_bst(option_root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root) = option_root.as_ref() {
        let node = root.borrow();
        if node.val < low {
            return trim_bst(node.right.clone(), low, high);
        } else if node.val > high {
            return trim_bst(node.left.clone(), low, high);
        }
        let left = trim_bst(node.left.clone(), low, high);
        let right = trim_bst(node.right.clone(), low, high);
        drop(node);
        root.borrow_mut().left = left;
        root.borrow_mut().right = right;
        return option_root.clone();
    }
    None
}

fn main() {
    assert_eq!(trim_bst(tree![1,0,2], 1, 2), tree![1,null,2]);
    assert_eq!(trim_bst(tree![3,0,4,null,2,null,null,1], 1, 3), tree![3,2,null,1]);
}
