//! 左叶子之和

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    let mut q = VecDeque::new();
    q.push_back(root);
    let mut ans = 0;
    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        let node = n.as_ref().unwrap().borrow();
        if node.left.is_some() {
            q.push_back(node.left.clone());
            if node.left.as_ref().unwrap().borrow().left.is_none() && node.left.as_ref().unwrap().borrow().right.is_none() {
                ans += node.left.as_ref().unwrap().borrow().val;
            }
        }
        if node.right.is_some() {
            q.push_back(node.right.clone());
        }
    }

    ans
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![3,9,20,null,null,15,7]), 24);
        assert_eq!(func(tree![1]), 0);
    }
    test(sum_of_left_leaves);
}
