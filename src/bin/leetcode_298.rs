//! 二叉树最长连续序列

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32, cur: i32) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        if node.left.is_some() {
            if node.left.as_ref().unwrap().borrow().val == node.val + 1 {
                *result = (*result).max(cur + 1);
                dfs(node.left.clone(), result, cur + 1);
            } else {
                dfs(node.left.clone(), result, 1);
            }
        }
        if node.right.is_some() {
            if node.right.as_ref().unwrap().borrow().val == node.val + 1 {
                *result = (*result).max(cur + 1);
                dfs(node.right.clone(), result, cur + 1);
            } else {
                dfs(node.right.clone(), result, 1);
            }
        }
    }
    let mut result = 1;
    dfs(root, &mut result, 1);
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![1]), 1);
        assert_eq!(func(tree![1,null,3,2,4,null,null,null,5]), 3);
        assert_eq!(func(tree![2,null,3,2,null,1]), 2);
    }
    test(longest_consecutive);
}
