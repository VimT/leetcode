//! 均匀树划分

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn check_equal_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn post(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        let left = post(root.as_ref().unwrap().borrow().left.clone());
        let right = post(root.as_ref().unwrap().borrow().right.clone());
        root.as_ref().unwrap().borrow_mut().val += left + right;
        root.as_ref().unwrap().borrow().val
    }
    fn pre(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() { return false; }
        let node = root.as_ref().unwrap().borrow();
        if sum - node.val == node.val { return true; }
        pre(node.left.clone(), sum) | pre(node.right.clone(), sum)
    }
    let sum = post(root.clone());
    let node = root.as_ref().unwrap().borrow();
    pre(node.left.clone(), sum) | pre(node.right.clone(), sum)
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> bool) {
        assert_eq!(func(tree![1,-1]), false);
        assert_eq!(func(tree![5,10,10,null,null,2,3]), true);
        assert_eq!(func(tree![1,2,10,null,null,2,20]), false);
    }
    test(check_equal_tree);
}
