//! 打家劫舍 III

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() { return (0, 0); }
        let node = root.as_ref().unwrap().borrow();
        let (ls, ln) = dfs(node.left.clone());
        let (rs, rn) = dfs(node.right.clone());
        return (node.val + ln + rn, ls.max(ln) + rs.max(rn));
    }
    let (a, b) = dfs(root);
    a.max(b)
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![3,2,3,null,3,null,1]), 7);
        assert_eq!(func(tree![3,4,5,1,3,null,1]), 9);
    }
    test(rob);
}
