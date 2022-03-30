//! 最长同值路径

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_lenth_path: &mut i32) -> i32 {
        if let Some(root) = root.as_ref() {
            let node = root.borrow();
            let mut guai_max = 0;
            let mut this_result = 0;
            if node.left.is_some() {
                let left = dfs(node.left.clone(), max_lenth_path);
                if node.val == node.left.as_ref().unwrap().borrow().val {
                    guai_max += left + 1;
                    this_result = this_result.max(left + 1);
                }
            }
            if node.right.is_some() {
                let right = dfs(node.right.clone(), max_lenth_path);
                if node.val == node.right.as_ref().unwrap().borrow().val {
                    guai_max += right + 1;
                    this_result = this_result.max(right + 1);
                }
            }
            *max_lenth_path = (*max_lenth_path).max(guai_max).max(this_result);
            return this_result;
        }
        0
    }
    let mut result = 0;
    dfs(root, &mut result);
    result
}


fn main() {
    assert_eq!(longest_univalue_path(tree![5,4,5,1,1,5]), 2);
    assert_eq!(longest_univalue_path(tree![1,4,5,4,4,5]), 2);
}
