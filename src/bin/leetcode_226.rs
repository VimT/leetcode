//! 翻转二叉树

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() { return root; }
    let right = invert_tree(root.as_ref().unwrap().borrow().left.clone());
    let left = invert_tree(root.as_ref().unwrap().borrow().right.clone());
    root.as_ref().unwrap().borrow_mut().right = right;
    root.as_ref().unwrap().borrow_mut().left = left;
    root
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(tree![4,2,7,1,3,6,9]), tree![4,7,2,9,6,3,1]);
        assert_eq!(func(tree![2,1,3]), tree![2,3,1]);
        assert_eq!(func(tree![]), tree![]);
    }
    test(invert_tree);
}
