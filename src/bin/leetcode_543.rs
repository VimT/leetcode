//! 二叉树的直径


use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        if let Some(v) = root.as_ref() {
            let node = v.borrow();
            let left = dfs(node.left.clone(), result);
            let right = dfs(node.right.clone(), result);
            let depth = left.max(right) + 1;
            *result = (*result).max(left + right);
            return depth;
        }
        0
    }
    let mut result = 0;
    dfs(root, &mut result);
    result
}

fn main() {
    assert_eq!(diameter_of_binary_tree(tree![1,2,3,4,5]), 3);
    assert_eq!(diameter_of_binary_tree(tree![1,2]), 1);
}
