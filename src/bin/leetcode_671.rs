//! 二叉树中第二小的节点

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return -1; }
        let node = root.as_ref().unwrap().borrow();
        if node.left.is_some() {
            let mut left = node.left.as_ref().unwrap().borrow().val;
            let mut right = node.right.as_ref().unwrap().borrow().val;
            if left == node.val {
                left = dfs(node.left.clone());
            }
            if right == node.val {
                right = dfs(node.right.clone());
            }
            return match (left != -1 && left > node.val, right != -1 && right > node.val) {
                (true, true) => left.min(right),
                (true, false) => left,
                (false, true) => right,
                (false, false) => -1
            };
        }
        -1
    }
    dfs(root)
}


fn main() {
    assert_eq!(find_second_minimum_value(tree![1,1,3,1,1,3,4,3,1,1,1,3,8,4,8,3,3,1,6,2,1]), 2);
    assert_eq!(find_second_minimum_value(tree![2,2,5,null,null,5,7]), 5);
    assert_eq!(find_second_minimum_value(tree![2,2,2]), -1);
}
