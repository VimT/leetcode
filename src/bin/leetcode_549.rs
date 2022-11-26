//! 二叉树中最长的连续序列

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (i32, i32) {
        if root.is_none() { return (0, 0); }
        let node = root.as_ref().unwrap().borrow();
        let (left_start, left_end) = dfs(node.left.clone(), result);
        let (right_start, right_end) = dfs(node.right.clone(), result);
        let mut start = 1;
        let mut end = 1;
        let left_val = node.left.as_ref().map(|x| x.borrow().val);
        let right_val = node.right.as_ref().map(|x| x.borrow().val);
        if let Some(left) = left_val {
            if left + 1 == node.val {
                end = end.max(left_end + 1);
            }
            if left == node.val + 1 {
                start = start.max(left_start + 1);
            }
        }
        if let Some(right) = right_val {
            if right + 1 == node.val {
                end = end.max(right_end + 1);
            }
            if right == node.val + 1 {
                start = start.max(right_start + 1);
            }
        }
        if let (Some(left), Some(right)) = (left_val, right_val) {
            if left + 1 == node.val && node.val + 1 == right {
                *result = (*result).max(left_end + 1 + right_start);
            }
            if right + 1 == node.val && node.val + 1 == left {
                *result = (*result).max(left_start + 1 + right_end);
            }
        }
        (*result) = (*result).max(start).max(end);
        (start, end)
    }
    let mut result = 0;
    dfs(root, &mut result);
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![1,2,3]), 2);
        assert_eq!(func(tree![2,1,3]), 3);
    }
    test(longest_consecutive);
}
