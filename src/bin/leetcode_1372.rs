//! 二叉树中的最长交错路径

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;

pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (i32, i32) {
        if root.is_none() { return (0, 0); }
        let node = root.as_ref().unwrap().borrow();
        let (_, left2) = dfs(node.left.clone(), result);
        let (right1, _) = dfs(node.right.clone(), result);
        *result = (*result).max(left2.max(right1));
        (left2 + 1, right1 + 1)
    }
    let mut result = 0;
    dfs(root, &mut result);
    result
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![1,null,1,1,1,null,null,1,1,null,1,null,null,null,1,null,1]), 3);
        assert_eq!(func(tree![1,1,1,null,1,null,null,1,1,null,1]), 4);
        assert_eq!(func(tree![1]), 0);
    }
    test(longest_zig_zag);
}
