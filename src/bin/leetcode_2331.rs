//! 计算布尔二叉树的值

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.as_ref().unwrap().borrow();
        return match node.val {
            0 => false,
            1 => true,
            2 => dfs(node.left.clone()) || dfs(node.right.clone()),
            3 => dfs(node.left.clone()) && dfs(node.right.clone()),
            _ => unreachable!()
        };
    }
    dfs(root)
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> bool) {
        assert_eq!(func(tree![2,1,3,null,null,0,1]), true);
        assert_eq!(func(tree![0]), false);
    }
    test(evaluate_tree);
}
